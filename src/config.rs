use std::env;

pub struct AppConfig {
    pub deepl_api_key: String,
    pub openweather_api_key: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            deepl_api_key: env::var("DEEPL_API_KEY")
                .expect("DEEPL_API_KEY не найден в .env"),
            openweather_api_key: env::var("OPENWEATHER_API_KEY")
                .expect("OPENWEATHER_API_KEY не найден в .env"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL не найден в .env"),
        }
    }
}
