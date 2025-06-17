use meval::eval_str;
use regex::Regex;

/// Определяет, содержит ли строка математическое выражение
pub fn is_math_expression(input: &str) -> bool {
    let input_clean = input.trim();
    
    // Проверяем наличие математических операторов
    let math_keywords = ["+", "-", "*", "/", "=", "^", "sqrt", "mod", "sin", "cos", "tan", "log", "ln"];
    let has_operators = math_keywords.iter().any(|kw| input_clean.contains(kw));
    
    // Проверяем наличие чисел
    let has_numbers = input_clean.chars().any(|c| c.is_ascii_digit());
    
    // Простые математические паттерны
    let math_patterns = [
        r"\d+\s*[\+\-\*/\^]\s*\d+",  // 2 + 3, 5 * 8
        r"\d+\s*=\s*\?",             // x = ?
        r"sqrt\(\d+\)",              // sqrt(16)
        r"\d+\s*mod\s*\d+",          // 10 mod 3
    ];
    
    let has_math_pattern = math_patterns.iter().any(|pattern| {
        Regex::new(pattern).unwrap().is_match(input_clean)
    });
    
    has_operators && has_numbers || has_math_pattern
}

/// Вычисляет значение математического выражения
pub fn evaluate_expression(input: &str) -> Result<f64, String> {
    let cleaned = clean_math_expression(input);
    eval_str(&cleaned).map_err(|e| format!("Ошибка вычисления: {}", e))
}

/// Очищает математическое выражение от лишних символов
fn clean_math_expression(input: &str) -> String {
    let mut cleaned = input.trim().to_string();
    
    // Убираем текстовые части
    cleaned = cleaned.replace("вычисли", "");
    cleaned = cleaned.replace("посчитай", "");
    cleaned = cleaned.replace("calculate", "");
    cleaned = cleaned.replace("равно", "");
    cleaned = cleaned.replace("equals", "");
    cleaned = cleaned.replace("=", "");
    cleaned = cleaned.replace("?", "");
    
    // Заменяем русские операторы
    cleaned = cleaned.replace(" плюс ", "+");
    cleaned = cleaned.replace(" минус ", "-");
    cleaned = cleaned.replace(" умножить на ", "*");
    cleaned = cleaned.replace(" разделить на ", "/");
    cleaned = cleaned.replace(" в степени ", "^");
    
    cleaned.trim().to_string()
}

/// Форматирует результат вычисления
pub fn format_math_result(expression: &str, result: f64) -> String {
    let cleaned_expr = clean_math_expression(expression);
    
    // Проверяем, является ли результат целым числом
    if result.fract() == 0.0 && result.abs() < 1e15 {
        format!("🧮 **{}** = **{}**", cleaned_expr, result as i64)
    } else {
        format!("🧮 **{}** = **{:.6}**", cleaned_expr, result)
    }
}

/// Решает простые математические задачи с объяснением
pub fn solve_with_explanation(input: &str) -> Result<String, String> {
    let result = evaluate_expression(input)?;
    let explanation = generate_explanation(input, result);
    Ok(format!("{}\n\n💡 {}", format_math_result(input, result), explanation))
}

/// Генерирует объяснение для математического решения
fn generate_explanation(expression: &str, result: f64) -> String {
    let cleaned = clean_math_expression(expression);
    
    if cleaned.contains("+") {
        "Выполнили сложение чисел."
    } else if cleaned.contains("-") {
        "Выполнили вычитание чисел."
    } else if cleaned.contains("*") {
        "Выполнили умножение чисел."
    } else if cleaned.contains("/") {
        if result.fract() == 0.0 {
            "Выполнили деление нацело."
        } else {
            "Выполнили деление с остатком."
        }
    } else if cleaned.contains("^") {
        "Возвели число в степень."
    } else if cleaned.contains("sqrt") {
        "Извлекли квадратный корень."
    } else {
        "Выполнили математическое вычисление."
    }.to_string()
}

/// Проверяет математические константы
pub fn get_math_constant(name: &str) -> Option<f64> {
    match name.to_lowercase().as_str() {
        "pi" | "пи" => Some(std::f64::consts::PI),
        "e" => Some(std::f64::consts::E),
        "tau" | "тау" => Some(std::f64::consts::TAU),
        _ => None
    }
}

/// Конвертирует единицы измерения
pub fn convert_units(value: f64, from: &str, to: &str) -> Option<(f64, String)> {
    match (from.to_lowercase().as_str(), to.to_lowercase().as_str()) {
        // Температура
        ("c", "f") | ("цельсий", "фаренгейт") => {
            Some((value * 9.0 / 5.0 + 32.0, format!("{:.1}°C = {:.1}°F", value, value * 9.0 / 5.0 + 32.0)))
        }
        ("f", "c") | ("фаренгейт", "цельсий") => {
            Some(((value - 32.0) * 5.0 / 9.0, format!("{:.1}°F = {:.1}°C", value, (value - 32.0) * 5.0 / 9.0)))
        }
        
        // Длина
        ("m", "ft") | ("метр", "фут") => {
            Some((value * 3.28084, format!("{:.2}m = {:.2}ft", value, value * 3.28084)))
        }
        ("ft", "m") | ("фут", "метр") => {
            Some((value / 3.28084, format!("{:.2}ft = {:.2}m", value, value / 3.28084)))
        }
        
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_math_expression() {
        assert!(is_math_expression("2 + 3"));
        assert!(is_math_expression("sqrt(16)"));
        assert!(is_math_expression("10 * 5"));
        assert!(!is_math_expression("Hello world"));
    }

    #[test]
    fn test_evaluate_expression() {
        assert_eq!(evaluate_expression("2 + 3").unwrap(), 5.0);
        assert_eq!(evaluate_expression("10 * 2").unwrap(), 20.0);
        assert_eq!(evaluate_expression("sqrt(16)").unwrap(), 4.0);
    }

    #[test]
    fn test_clean_expression() {
        assert_eq!(clean_math_expression("вычисли 2 + 3"), "2 + 3");
        assert_eq!(clean_math_expression("5 плюс 3"), "5+3");
    }

    #[test]
    fn test_math_constants() {
        assert!(get_math_constant("pi").is_some());
        assert!(get_math_constant("e").is_some());
        assert!(get_math_constant("unknown").is_none());
    }
}
