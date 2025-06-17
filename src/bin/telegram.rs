// src/bin/telegram.rs

use bot_core::integration::telegram::bot::run_bot;

#[tokio::main]
async fn main() {
    // Загружаем переменные окружения из .env файла
    dotenv::dotenv().ok();
    
    // Инициализация логирования
    env_logger::init();
    
    println!("🚀 Запуск Telegram-бота Auraya...");
    println!("⚠️ Работаем без базы данных (режим тестирования)");
    
    // Проверяем наличие токена
    match std::env::var("TELOXIDE_TOKEN") {
        Ok(token) => {
            println!("✅ Токен найден: {}...{}", &token[..10], &token[token.len()-10..]);
        }
        Err(_) => {
            eprintln!("❌ TELOXIDE_TOKEN не найден в .env файле");
            return;
        }
    }
    
    // Запуск бота
    println!("🤖 Подключение к Telegram API...");
    run_bot().await;
}
