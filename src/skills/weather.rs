use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherCondition>,
    main: WeatherMain,
    name: String,
}

#[derive(Debug, Deserialize)]
struct WeatherCondition {
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f64,
    humidity: u8,
}

/// Получает текущую погоду по названию города
pub fn get_weather(city: &str) -> Result<String, String> {
    let api_key = env::var("OPENWEATHER_API_KEY").map_err(|_| "API key не найден".to_string())?;
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=ru",
        city, api_key
    );

    let client = Client::new();
    let res = client
        .get(&url)
        .send()
        .map_err(|e| format!("Ошибка запроса: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Ошибка: {}", res.status()));
    }

    let data: WeatherResponse = res
        .json()
        .map_err(|e| format!("Ошибка разбора JSON: {}", e))?;

    let fallback = String::from("?");
    let description = data.weather.first().map(|w| &w.description).unwrap_or(&fallback);

    let response = format!(
        "Погода в {}: {}, температура {}°C, влажность {}%",
        data.name, description, data.main.temp, data.main.humidity
    );

    Ok(response)
}
