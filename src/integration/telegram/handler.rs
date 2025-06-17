use teloxide::{prelude::*, types::Message, utils::command::BotCommands};
use crate::{
    integration::telegram::bot::Command,
    core::{dialog::DialogContext, logic::smart_answer_multilang},
};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::OnceLock;

// Глобальное хранилище состояний диалогов для каждого чата
static CHAT_STATES: OnceLock<Mutex<HashMap<ChatId, DialogContext>>> = OnceLock::new();

fn get_chat_states() -> &'static Mutex<HashMap<ChatId, DialogContext>> {
    CHAT_STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Обрабатывает команды бота
pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    match cmd {
        Command::Help => {
            let help_text = Command::descriptions().to_string();
            bot.send_message(chat_id, help_text).await?;
        }

        Command::Start => {
            let mut states = get_chat_states().lock().await;
            states.insert(chat_id, DialogContext::new());
            
            let welcome_text = "👋 Привет! Я Auraya - умный ассистент.\n\n\
                Я могу помочь вам с:\n\
                • Поиском информации в Wikipedia\n\
                • Поиском репозиториев на GitHub\n\
                • Переводом текста\n\
                • И многим другим!\n\n\
                Теперь можете просто писать мне сообщения без команд! 💬\n\
                Или используйте команды:\n\
                /ask <вопрос> - задать конкретный вопрос\n\
                /help - показать справку";
            
            bot.send_message(chat_id, welcome_text).await?;
        }

        Command::Ask(question) => {
            if question.trim().is_empty() {
                bot.send_message(chat_id, "❓ Пожалуйста, укажите ваш вопрос после команды /ask")
                    .await?;
                return Ok(());
            }

            let mut states = get_chat_states().lock().await;
            let dialog = states.entry(chat_id).or_insert_with(DialogContext::new);
            
            let reply = smart_answer_multilang(&question, dialog).await;
            bot.send_message(chat_id, reply).await?;
        }
    }

    Ok(())
}

/// Обрабатывает обычные текстовые сообщения (без команд)
pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    
    // Получаем текст сообщения
    if let Some(text) = msg.text() {
        // Показываем, что бот печатает
        bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
        
        // Получаем или создаем контекст диалога
        let mut states = get_chat_states().lock().await;
        let dialog = states.entry(chat_id).or_insert_with(DialogContext::new);
        
        // Генерируем ответ как ИИ-ассистент
        let reply = smart_answer_multilang(text, dialog).await;
        
        bot.send_message(chat_id, reply).await?;
    }

    Ok(())
}
