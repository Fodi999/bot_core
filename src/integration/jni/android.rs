use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

use crate::core::dialog::handle_input;

/// Функция, вызываемая из Kotlin/Java через JNI
#[no_mangle]
pub extern "system" fn Java_com_example_botcore_Bot_handleInput(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    // Преобразуем JString → Rust String
    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Ошибка ввода").unwrap().into_raw(),
    };

    // Вызываем ядро бота
    let reply = match handle_input(input_str) {
        Ok(response) => response,
        Err(_) => "Ошибка обработки".to_string(),
    };

    // Преобразуем результат в jstring и возвращаем
    env.new_string(reply).unwrap().into_raw()
}
