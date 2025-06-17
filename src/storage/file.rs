use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::error::Error;

/// Сохраняет строку в файл по указанному пути
pub fn save_to_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Загружает содержимое файла как строку
pub fn load_from_file(path: &str) -> Result<String, Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Err("Файл не найден".into());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Удаляет файл, если он существует
pub fn delete_file(path: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(path).exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
