use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;
use tokio::sync::OnceCell;

static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_db() -> &'static Pool<Postgres> {
    DB.get_or_init(|| async {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL не найден в .env");
        
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Ошибка подключения к базе данных")
    }).await
}

// Функция для получения данных из кэша
pub async fn get_from_cache(key: &str) -> Option<String> {
    let db = match get_db().await {
        pool => pool,
    };
    
    let query = "SELECT value FROM cache WHERE key = $1 AND expires_at > NOW()";
    
    match sqlx::query_scalar::<_, String>(query)
        .bind(key)
        .fetch_optional(db)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Ошибка получения из кэша: {}", e);
            None
        }
    }
}

// Функция для сохранения данных в кэш
pub async fn save_to_cache(key: &str, value: &str) {
    let db = match get_db().await {
        pool => pool,
    };
    
    let query = r#"
        INSERT INTO cache (key, value, expires_at) 
        VALUES ($1, $2, NOW() + INTERVAL '1 hour')
        ON CONFLICT (key) 
        DO UPDATE SET value = EXCLUDED.value, expires_at = EXCLUDED.expires_at
    "#;
    
    if let Err(e) = sqlx::query(query)
        .bind(key)
        .bind(value)
        .execute(db)
        .await
    {
        eprintln!("Ошибка сохранения в кэш: {}", e);
    }
}

// Функция для инициализации таблицы кэша
pub async fn init_cache_table() -> Result<(), sqlx::Error> {
    let db = match get_db().await {
        pool => pool,
    };
    
    // Создаем таблицу
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS cache (
            key VARCHAR(255) PRIMARY KEY,
            value TEXT NOT NULL,
            expires_at TIMESTAMP NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#;
    
    sqlx::query(create_table_query)
        .execute(db)
        .await?;
    
    // Создаем индекс
    let create_index_query = r#"
        CREATE INDEX IF NOT EXISTS idx_cache_expires_at ON cache(expires_at)
    "#;
    
    sqlx::query(create_index_query)
        .execute(db)
        .await?;
    
    Ok(())
}

// Функция для очистки просроченных записей кэша
pub async fn cleanup_expired_cache() -> Result<u64, sqlx::Error> {
    let db = match get_db().await {
        pool => pool,
    };
    
    let query = "DELETE FROM cache WHERE expires_at <= NOW()";
    
    let result = sqlx::query(query)
        .execute(db)
        .await?;
    
    Ok(result.rows_affected())
}

