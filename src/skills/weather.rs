use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherCondition>,
    main: WeatherMain,
    name: String,
    sys: Option<WeatherSys>,
    wind: Option<WeatherWind>,
}

#[derive(Debug, Deserialize)]
struct WeatherCondition {
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f64,
    feels_like: f64,
    humidity: u8,
    pressure: u16,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherSys {
    country: String,
}

#[derive(Debug, Deserialize)]
struct WeatherWind {
    speed: f64,
}

/// Проверяет, является ли запрос погодным
pub fn is_weather_query(input: &str) -> bool {
    let input_lower = input.to_lowercase();
    let weather_keywords = [
        "погода", "weather", "температура", "temperature", 
        "дождь", "rain", "снег", "snow", "солнце", "sunny",
        "облачно", "cloudy", "туман", "fog", "ветер", "wind"
    ];
    
    weather_keywords.iter().any(|kw| input_lower.contains(kw))
}

/// Извлекает название города из запроса
pub fn extract_city_from_query(input: &str) -> Option<String> {
    let input_lower = input.to_lowercase();
    
    // Паттерны для поиска города
    let patterns = [
        r"погода в (.+)",
        r"weather in (.+)", 
        r"температура в (.+)",
        r"какая погода в (.+)",
        r"how is weather in (.+)"
    ];
    
    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(&input_lower) {
                if let Some(city) = captures.get(1) {
                    return Some(city.as_str().trim().to_string());
                }
            }
        }
    }
    
    None
}

/// Получает текущую погоду по названию города (асинхронная версия)
pub async fn get_weather(city: &str) -> Result<String, String> {
    // Проверяем наличие API ключа
    let api_key = match env::var("OPENWEATHER_API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            return Err("⚠️ OpenWeather API ключ не найден. Добавьте OPENWEATHER_API_KEY в .env файл".to_string());
        }
    };
    
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=ru",
        urlencoding::encode(city), api_key
    );

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Ошибка создания HTTP клиента: {}", e))?;

    let response = client
        .get(&url)
        .header("User-Agent", "Bot-Auraya/1.0")
        .send()
        .await
        .map_err(|e| format!("Ошибка запроса погоды: {}", e))?;

    if !response.status().is_success() {
        return match response.status().as_u16() {
            404 => Err(format!("🏙️ Город **{}** не найден. Проверьте правильность написания.", city)),
            401 => Err("🔑 Неверный API ключ OpenWeather".to_string()),
            429 => Err("⏰ Превышен лимит запросов к API погоды".to_string()),
            _ => Err(format!("❌ Ошибка получения погоды: {}", response.status()))
        };
    }

    let data: WeatherResponse = response
        .json()
        .await
        .map_err(|e| format!("Ошибка разбора ответа погоды: {}", e))?;

    Ok(format_weather_response(&data))
}

/// Форматирует ответ о погоде с эмодзи и подробной информацией
fn format_weather_response(data: &WeatherResponse) -> String {
    let weather_condition = data.weather.first();
    let description = weather_condition
        .map(|w| w.description.as_str())
        .unwrap_or("неизвестно");
    
    let weather_emoji = get_weather_emoji(weather_condition);
    let temp_emoji = get_temperature_emoji(data.main.temp);
    
    let mut response = format!(
        "{} **Погода в {}{}**\n\n",
        weather_emoji,
        data.name,
        data.sys.as_ref().map(|s| format!(", {}", s.country)).unwrap_or_default()
    );
    
    response.push_str(&format!(
        "🌡️ **Температура:** {:.1}°C (ощущается как {:.1}°C) {}\n",
        data.main.temp, data.main.feels_like, temp_emoji
    ));
    
    response.push_str(&format!(
        "📊 **Состояние:** {}\n",
        capitalize_first_letter(description)
    ));
    
    response.push_str(&format!(
        "💧 **Влажность:** {}%\n",
        data.main.humidity
    ));
    
    response.push_str(&format!(
        "📏 **Давление:** {} мм рт.ст.\n",
        (data.main.pressure as f64 * 0.75).round() as u16
    ));
    
    if let Some(wind) = &data.wind {
        response.push_str(&format!(
            "💨 **Ветер:** {:.1} м/с\n",
            wind.speed
        ));
    }
    
    response.push_str(&format!(
        "📈 **Диапазон:** {:.1}°C ... {:.1}°C",
        data.main.temp_min, data.main.temp_max
    ));
    
    response
}

/// Получает эмодзи для погодных условий
fn get_weather_emoji(weather: Option<&WeatherCondition>) -> &'static str {
    match weather.map(|w| w.main.as_str()) {
        Some("Clear") => "☀️",
        Some("Clouds") => "☁️", 
        Some("Rain") => "🌧️",
        Some("Drizzle") => "🌦️",
        Some("Thunderstorm") => "⛈️",
        Some("Snow") => "❄️",
        Some("Mist") | Some("Fog") => "🌫️",
        _ => "🌤️"
    }
}

/// Получает эмодзи для температуры
fn get_temperature_emoji(temp: f64) -> &'static str {
    match temp {
        t if t >= 30.0 => "🔥",
        t if t >= 25.0 => "🌡️",
        t if t >= 15.0 => "🌤️",
        t if t >= 5.0 => "❄️",
        t if t >= -10.0 => "🧊",
        _ => "🥶"
    }
}

/// Делает первую букву заглавной
fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Предоставляет информацию о погоде без API (fallback)
pub fn get_weather_fallback(city: &str) -> String {
    format!(
        "🌤️ **Информация о погоде в {}**\n\n\
        К сожалению, актуальные данные недоступны.\n\n\
        💡 **Для получения погоды:**\n\
        • Добавьте OPENWEATHER_API_KEY в .env\n\
        • Зарегистрируйтесь на openweathermap.org\n\
        • Получите бесплатный API ключ\n\n\
        📱 **Альтернативы:** Яндекс.Погода, AccuWeather, Weather.com",
        city
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_weather_query() {
        assert!(is_weather_query("Какая погода в Москве?"));
        assert!(is_weather_query("Weather in London"));
        assert!(is_weather_query("температура сегодня"));
        assert!(!is_weather_query("Hello world"));
    }

    #[test]
    fn test_extract_city() {
        assert_eq!(extract_city_from_query("погода в москве"), Some("москве".to_string()));
        assert_eq!(extract_city_from_query("weather in london"), Some("london".to_string()));
        assert_eq!(extract_city_from_query("hello"), None);
    }

    #[test]
    fn test_weather_emoji() {
        assert_eq!(get_temperature_emoji(35.0), "🔥");
        assert_eq!(get_temperature_emoji(20.0), "🌤️");
        assert_eq!(get_temperature_emoji(-15.0), "🥶");
    }
}
