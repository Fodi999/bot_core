use teloxide::{prelude::*, utils::command::BotCommands};
use crate::integration::telegram::handler::handle_command;

/// Telegram-команды, доступные пользователю
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Список команд:")]
pub enum Command {
    #[command(description = "Показать справку")]
    Help,
    #[command(description = "Начать диалог")]
    Start,
    #[command(description = "Задать вопрос боту")]
    Ask(String),
}

/// Инициализация и запуск Telegram-бота
pub async fn run_bot() {
    log::info!("🤖 Telegram-бот запущен...");

    let bot = Bot::from_env();

    Command::repl(bot, handle_command).await;
}
