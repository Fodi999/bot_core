use teloxide::{prelude::*, utils::command::BotCommands};
use crate::integration::telegram::handler::{handle_command, handle_message};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Telegram-команды, доступные пользователю")]
pub enum Command {
    #[command(description = "Показать справку")]
    Help,
    #[command(description = "Начать диалог")]
    Start,
    #[command(description = "Задать вопрос боту")]
    Ask(String),
}

pub async fn run_bot() {
    // Убираем инициализацию логгера - она уже есть в main.rs
    log::info!("Запуск бота...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(handle_command), // Обработка команд
            )
            .branch(
                dptree::filter(|msg: Message| !msg.text().unwrap_or("").starts_with('/'))
                    .endpoint(handle_message), // Обработка обычных сообщений
            ),
    )
    .dependencies(dptree::deps![])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
