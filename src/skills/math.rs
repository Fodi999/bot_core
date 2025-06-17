use meval::eval_str;

/// Определяет, содержит ли строка математическое выражение
pub fn is_math_expression(input: &str) -> bool {
    let math_keywords = ["+", "-", "*", "/", "=", "^", "sqrt", "mod"];
    math_keywords.iter().any(|kw| input.contains(kw))
}

/// Вычисляет значение математического выражения
pub fn evaluate_expression(input: &str) -> Result<f64, String> {
    eval_str(input).map_err(|e| format!("Ошибка вычисления: {}", e))
}
