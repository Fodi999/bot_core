use crate::{
    core::dialog::DialogContext,
    knowledge::{wikipedia::fetch_wikipedia_summary, github::search_github_repos},
    language::{detect::detect_language, translator::translate_text},
    storage::db::{get_from_cache, save_to_cache},
};

/// Асинхронная функция, которая возвращает умный ответ с учётом языка пользователя
pub async fn smart_answer_multilang(user_input: &str, dialog: &mut DialogContext) -> String {
    dialog.add_message("user", user_input);

    // Определяем язык входного сообщения
    let lang = detect_language(user_input).unwrap_or_else(|| "EN".to_string());

    // Переводим на английский, если нужно
    let input_en = if lang != "EN" {
        match translate_text(user_input, "EN").await {
            Ok(translated) => translated,
            Err(e) => {
                println!("Ошибка перевода: {}", e);
                user_input.to_string()
            }
        }
    } else {
        user_input.to_string()
    };

    // Проверка кэша
    if let Some(cached) = get_from_cache(&input_en).await {
        let translated = translate_if_needed(&cached, &lang).await;
        dialog.add_message("bot", &translated);
        return translated;
    }

    // Основная логика ответа с улучшенной обработкой ошибок
    let response_en = if input_en.to_lowercase().starts_with("what is") || 
                         input_en.to_lowercase().contains("что такое") {
        
        println!("Запрос Wikipedia: {}", input_en);
        match fetch_wikipedia_summary(&input_en).await {
            Ok(summary) if !summary.is_empty() && summary != "No summary found." => {
                println!("Wikipedia ответ получен");
                summary
            }
            Ok(_) => {
                println!("Wikipedia: пустой ответ");
                generate_fallback_response(&input_en)
            }
            Err(e) => {
                println!("Ошибка Wikipedia: {}", e);
                generate_fallback_response(&input_en)
            }
        }
    } else if input_en.to_lowercase().contains("rust example") || 
              input_en.to_lowercase().contains("github") {
        
        println!("Запрос GitHub: {}", input_en);
        match search_github_repos("rust example", 3).await {
            Ok(repos) if !repos.is_empty() => {
                println!("GitHub: найдено {} репозиториев", repos.len());
                repos.into_iter()
                    .map(|(name, description, url)| {
                        format!("📂 **{}**\n{}\n🔗 {}", name, description, url)
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n")
            }
            Ok(_) => {
                println!("GitHub: репозитории не найдены");
                "К сожалению, примеры кода сейчас недоступны. Попробуйте позже.".to_string()
            }
            Err(e) => {
                println!("Ошибка GitHub: {}", e);
                "Ошибка поиска репозиториев. Попробуйте позже.".to_string()
            }
        }
    } else {
        // Простые ответы на часто задаваемые вопросы
        generate_simple_response(&input_en)
    };

    // Сохраняем ответ в кэш и переводим обратно
    save_to_cache(&input_en, &response_en).await;
    let final_response = translate_if_needed(&response_en, &lang).await;
    dialog.add_message("bot", &final_response);
    final_response
}

/// Генерирует резервный ответ когда внешние API недоступны
fn generate_fallback_response(query: &str) -> String {
    if query.to_lowercase().contains("rust") {
        "🦀 **Rust** - это системный язык программирования, известный своей безопасностью памяти и высокой производительностью. \
        Rust используется для создания операционных систем, веб-серверов, блокчейна и многого другого.".to_string()
    } else if query.to_lowercase().contains("artificial intelligence") || 
              query.to_lowercase().contains("ai") {
        "🤖 **Искусственный интеллект (ИИ)** - это область компьютерных наук, которая создает системы, способные выполнять задачи, \
        обычно требующие человеческого интеллекта, такие как распознавание речи, принятие решений и обучение.".to_string()
    } else if query.to_lowercase().contains("programming") {
        "💻 **Программирование** - это процесс создания компьютерных программ с помощью языков программирования. \
        Это включает в себя написание кода, отладку и тестирование программного обеспечения.".to_string()
    } else {
        "Извините, информация временно недоступна из-за сетевых ограничений. Попробуйте переформулировать вопрос или попробовать позже.".to_string()
    }
}

/// Генерирует простые ответы на общие вопросы
fn generate_simple_response(input: &str) -> String {
    let input_lower = input.to_lowercase();
    
    if input_lower.contains("hello") || input_lower.contains("привет") {
        "👋 Привет! Как дела? Чем могу помочь?".to_string()
    } else if input_lower.contains("how are you") || input_lower.contains("как дела") {
        "Отлично! Готов помочь с любыми вопросами. 😊".to_string()
    } else if input_lower.contains("thank") || input_lower.contains("спасибо") {
        "Пожалуйста! Рад был помочь! 😊".to_string()
    } else {
        "Интересный вопрос! К сожалению, пока не могу дать развернутый ответ. \
        Попробуйте спросить о программировании, технологиях или используйте команды /help для списка возможностей.".to_string()
    }
}

/// Переводит текст, если требуется, на язык пользователя
async fn translate_if_needed(text: &str, target_lang: &str) -> String {
    if target_lang != "EN" {
        match translate_text(text, target_lang).await {
            Ok(translated) => translated,
            Err(e) => {
                println!("Ошибка перевода ответа: {}", e);
                text.to_string()
            }
        }
    } else {
        text.to_string()
    }
}


