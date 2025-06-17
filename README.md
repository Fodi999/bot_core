# 🤖 AurayaBot – Умный кроссплатформенный чат-бот на Rust

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![Telegram](https://img.shields.io/badge/Telegram-Bot-blue?logo=telegram)
![Platform](https://img.shields.io/badge/Platform-iOS%20%7C%20Android%20%7C%20Web%20%7C%20CLI-green)
![License](https://img.shields.io/badge/license-MIT-brightgreen)

> **Auraya** — это умный бот-помощник с памятью, логикой, знаниями и навыками. Он работает в Telegram, на мобильных и веб-платформах, а также умеет обучаться и взаимодействовать с реальным интернетом.

---

## 🚀 Возможности

- 🧠 Диалоговая логика с памятью и планированием
- 🌍 Поддержка всех языков через DeepL API
- 📚 Доступ к знаниям: Wikipedia, GitHub, Arxiv
- 🤖 Навыки: код, математика, погода, интеллект
- 📦 Подключение к PostgreSQL (Neon)
- 🔌 Кроссплатформенность (Telegram, iOS, Android, Web)
- 🧩 Расширяемая архитектура: легко добавлять новые модули

---

## 🗂 Структура проекта
bot_core/
├── src/
│ ├── core/ # Диалог, логика, память, планирование
│ ├── integration/ # Telegram, CLI, iOS, Android, WebAssembly
│ ├── knowledge/ # Wikipedia, GitHub, Arxiv, Web-парсинг
│ ├── language/ # Перевод, определение, нормализация языка
│ ├── skills/ # Навыки: code, math, weather
│ ├── storage/ # База данных и файловое хранилище
│ ├── config.rs # Загрузка ENV переменных
│ ├── utils.rs # Вспомогательные функции
│ └── lib.rs # Подключение модулей
├── src/bin/telegram.rs # Точка запуска Telegram-бота
└── Cargo.toml # Зависимости и сборка





