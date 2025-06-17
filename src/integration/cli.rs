use std::io::{self, Write};
use crate::core::dialog::handle_input;

/// Запускает простой REPL-чат в терминале
pub fn run_cli() {
    println!("🤖 Чат-бот активен. Напиши что-нибудь (введите `exit` для выхода):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка ввода. Повторите.");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("👋 До встречи!");
            break;
        }

        match handle_input(input.to_string()) {
            Ok(reply) => println!("Бот: {}", reply),
            Err(_) => println!("Бот: [Ошибка обработки]"),
        }
    }
}
