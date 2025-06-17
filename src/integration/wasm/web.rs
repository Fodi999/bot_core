use wasm_bindgen::prelude::*;
use crate::core::dialog::handle_input;

/// Инициализация логов для отладки в браузере
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Основной обработчик — вызывается из JS
#[wasm_bindgen]
pub fn bot_handle_input(input: &str) -> String {
    match handle_input(input.to_string()) {
        Ok(result) => result,
        Err(_) => "Ошибка обработки".to_string(),
    }
}
