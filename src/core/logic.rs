use crate::{
    core::dialog::DialogContext,
    knowledge::{wikipedia::fetch_wikipedia_summary, github::search_github_repos},
    language::{detect::detect_language, translator::translate_text},
    storage::db::{get_from_cache, save_to_cache},
    skills::{
        code::{detect_code_query, fetch_code_examples},
        math::{is_math_expression, solve_with_explanation},
        weather::{is_weather_query, extract_city_from_query, get_weather, get_weather_fallback},
    },
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

    // Основная логика ответа с расширенными навыками
    let response_en = if input_en.to_lowercase().starts_with("what is") || 
                         input_en.to_lowercase().contains("что такое") {
        
        println!("📖 Запрос Wikipedia: {}", input_en);
        match fetch_wikipedia_summary(&input_en).await {
            Ok(summary) if !summary.is_empty() && summary != "No summary found." => {
                println!("✅ Wikipedia ответ получен");
                summary
            }
            Ok(_) => {
                println!("⚠️ Wikipedia: пустой ответ");
                generate_fallback_response(&input_en)
            }
            Err(e) => {
                println!("❌ Ошибка Wikipedia: {}", e);
                generate_fallback_response(&input_en)
            }
        }
    } 
    // Математические выражения - ИСПРАВЛЕНО: убрал .await
    else if is_math_expression(&input_en) {
        println!("🧮 Математическое выражение: {}", input_en);
        match solve_with_explanation(&input_en) {
            Ok(result) => result,
            Err(e) => {
                println!("❌ Ошибка вычисления: {}", e);
                format!("❌ Не могу вычислить: **{}**\n\n💡 Проверьте правильность выражения. Поддерживаются: +, -, *, /, ^, sqrt(), sin(), cos() и т.д.", input_en)
            }
        }
    }
    // Погодные запросы - НОВЫЙ НАВЫК  
    else if is_weather_query(&input_en) {
        if let Some(city) = extract_city_from_query(&input_en) {
            println!("🌤️ Запрос погоды для города: {}", city);
            match get_weather(&city).await {
                Ok(weather_info) => weather_info,
                Err(e) => {
                    println!("❌ Ошибка погоды: {}", e);
                    if e.contains("API ключ") {
                        get_weather_fallback(&city)
                    } else {
                        e
                    }
                }
            }
        } else {
            "🌤️ Укажите город для получения погоды!\n\nПример: \"Погода в Москве\" или \"Weather in London\" 🏙️".to_string()
        }
    }
    // Примеры кода - УЛУЧШЕННЫЙ НАВЫК
    else if let Some(language) = detect_code_query(&input_en) {
        println!("💻 Запрос примеров кода: {}", language);
        match fetch_code_examples(&language).await {
            Ok(examples) => examples,
            Err(e) => {
                println!("❌ Ошибка поиска кода: {}", e);
                format!("К сожалению, примеры кода для **{}** сейчас недоступны. Попробуйте позже! 💻", language)
            }
        }
    }
    // GitHub репозитории
    else if input_en.to_lowercase().contains("rust example") || 
            input_en.to_lowercase().contains("github") ||
            input_en.to_lowercase().contains("code") {
        
        println!("🔍 Запрос GitHub: {}", input_en);
        match search_github_repos("rust example", 3).await {
            Ok(repos) if !repos.is_empty() => {
                println!("✅ GitHub: найдено {} репозиториев", repos.len());
                repos.into_iter()
                    .map(|(name, description, url)| {
                        format!("📂 **{}**\n{}\n🔗 {}", name, description, url)
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n")
            }
            Ok(_) => {
                println!("⚠️ GitHub: репозитории не найдены");
                "К сожалению, примеры кода сейчас недоступны. Попробуйте позже! 💻".to_string()
            }
            Err(e) => {
                println!("❌ Ошибка GitHub: {}", e);
                "Извините, сейчас не могу найти репозитории. Попробуйте позже! 🔧".to_string()
            }
        }
    } 
    else {
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
        Rust используется для создания операционных систем, веб-серверов, блокчейна и многого другого!\n\n\
        К сожалению, подробную информацию из Wikipedia сейчас получить не удалось, но основы я знаю! 😊".to_string()
    } else if query.to_lowercase().contains("artificial intelligence") || 
              query.to_lowercase().contains("ai") {
        "🤖 **Искусственный интеллект (ИИ)** - это область компьютерных наук, которая создает системы, способные выполнять задачи, \
        обычно требующие человеческого интеллекта, такие как распознавание речи, принятие решений и обучение.\n\n\
        Извините, что не могу дать более подробную информацию из Wikipedia прямо сейчас! 🔧".to_string()
    } else if query.to_lowercase().contains("programming") || query.to_lowercase().contains("программирование") {
        "💻 **Программирование** - это процесс создания компьютерных программ с помощью языков программирования. \
        Это включает в себя написание кода, отладку и тестирование программного обеспечения.\n\n\
        Хотя точную информацию из Wikipedia сейчас получить не удалось, основы я могу объяснить! 😊".to_string()
    } else {
        "🤔 Интересный вопрос! К сожалению, сейчас не могу найти подробную информацию из внешних источников, \
        но попробуйте спросить более конкретно - например, о программировании, технологиях или науке. \
        Возможно, у меня есть базовые знания по этой теме! 💡".to_string()
    }
}

/// Генерирует простые ответы на общие вопросы и ведет диалог как ИИ
fn generate_simple_response(input: &str) -> String {
    let input_lower = input.to_lowercase();
    
    // Приветствия
    if input_lower.contains("hello") || input_lower.contains("привет") || input_lower.contains("hi") {
        "👋 Привет! Как дела? О чём поговорим?".to_string()
    } 
    // Состояние бота
    else if input_lower.contains("how are you") || input_lower.contains("как дела") {
        "Отлично! Готов помочь с любыми вопросами. Что тебя интересует? 😊".to_string()
    }
    // Благодарности
    else if input_lower.contains("thank") || input_lower.contains("спасибо") {
        "Пожалуйста! Рад был помочь! Есть ещё вопросы? 😊".to_string()
    }
    // Прощания
    else if input_lower.contains("bye") || input_lower.contains("пока") || input_lower.contains("до свидания") {
        "До свидания! Удачного дня! Обращайся, если что-то понадобится! 👋".to_string()
    }
    // Вопросы о боте - ОБНОВЛЕННЫЙ СПИСОК НАВЫКОВ
    else if input_lower.contains("what are you") || input_lower.contains("кто ты") || input_lower.contains("что ты") {
        "Я Auraya - умный ИИ-ассистент! 🤖 Могу помочь с поиском информации, ответить на вопросы о программировании, найти репозитории на GitHub, решить математические задачи, узнать погоду и многое другое. Чем могу быть полезен?".to_string()
    }
    // Помощь - ОБНОВЛЕННЫЙ СПИСОК
    else if input_lower.contains("help") || input_lower.contains("помощь") || input_lower.contains("что умеешь") {
        "Я умею:\n• 🔍 Искать информацию в Wikipedia\n• 💻 Находить репозитории на GitHub\n• 🧮 Решать математические задачи\n• 🌤️ Узнавать погоду в городах\n• 💡 Показывать примеры кода\n• 🌍 Переводить тексты\n• 💬 Поддерживать диалог на разных языках\n\nПросто задавай вопросы или используй команды /help!".to_string()
    }
    // Короткие сообщения
    else if input_lower.len() < 10 && !input_lower.chars().any(|c| c == '?' || c == '!') {
        let responses = [
            "Интересно! Расскажи подробнее 🤔",
            "Понял! А что именно тебя интересует?",
            "Хм, расскажи больше об этом!",
            "Интригующе! Продолжай 😊"
        ];
        responses[input.len() % responses.len()].to_string()
    }
    // Вопросы
    else if input_lower.contains('?') || input_lower.contains("как") || input_lower.contains("что") || input_lower.contains("where") || input_lower.contains("how") || input_lower.contains("what") {
        "Хороший вопрос! 🤔 Попробуй спросить более конкретно:\n\n• 📖 \"Что такое Rust?\" - поиск в Wikipedia\n• 🧮 \"2 + 2 * 3\" - математические вычисления\n• 🌤️ \"Погода в Москве\" - прогноз погоды\n• 💻 \"Примеры кода на Python\" - поиск репозиториев\n\nЯ постараюсь найти информацию!".to_string()
    }
    // Эмоции
    else if input_lower.contains("love") || input_lower.contains("люблю") {
        "Приятно слышать! 😊 А что именно тебя вдохновляет?".to_string()
    }
    else if input_lower.contains("hate") || input_lower.contains("ненавижу") {
        "Понимаю, бывают сложные моменты. Может, поговорим о чём-то более позитивном? 🌟".to_string()
    }
    // Общие темы - ОБНОВЛЕНЫ ДЛЯ НОВЫХ НАВЫКОВ
    else if input_lower.contains("weather") || input_lower.contains("погода") {
        "🌤️ Хочешь узнать погоду? Скажи мне город!\n\nПример: \"Погода в Москве\" или \"Weather in London\" 🏙️".to_string()
    }
    else if input_lower.contains("time") || input_lower.contains("время") {
        "Время лучше проверить на своём устройстве! ⏰ А у меня есть время помочь тебе с любыми вопросами!".to_string()
    }
    // Обучение и знания
    else if input_lower.contains("learn") || input_lower.contains("учить") || input_lower.contains("изучать") {
        "Обучение - это здорово! 📚 Что хочешь изучить?\n\n• 💻 Программирование - покажу примеры кода\n• 🧮 Математика - решу задачи\n• 🌍 Технологии - найдю информацию\n\nПросто спроси конкретно!".to_string()
    }
    // Длинные сообщения
    else if input.len() > 50 {
        format!("Понимаю, что ты говоришь о \"{}\". Это интересная тема! 🤔 Попробуй задать более конкретный вопрос - возможно, смогу помочь с:\n\n• 📖 Поиском информации\n• 🧮 Вычислениями\n• 💻 Примерами кода\n• 🌤️ Погодой", 
                &input[..input.len().min(40)])
    }
    // Общий ответ
    else {
        let responses = [
            "Интересно! Могу помочь с поиском информации, вычислениями или примерами кода 🤔",
            "Понял тебя! Попробуй спросить о погоде, математике или программировании 😊",
            "Хм, интригующая тема! Можешь задать более конкретный вопрос? 💡",
            "Это любопытно! Давай обсудим конкретные детали 🚀"
        ];
        responses[input.len() % responses.len()].to_string()
    }
}

/// Переводит текст, если требуется, на язык пользователя
async fn translate_if_needed(text: &str, target_lang: &str) -> String {
    if target_lang != "EN" {
        match translate_text(text, target_lang).await {
            Ok(translated) => translated,
            Err(e) => {
                println!("⚠️ Ошибка перевода ответа: {} | Используем fallback", e);
                // Можно добавить простой переводчик-заглушку
                text.to_string()
            }
        }
    } else {
        text.to_string()
    }
}


