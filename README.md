# Todo List Manager (Rust)

[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Консольное приложение для управления задачами с сохранением в SQLite базе данных.

## 🚀 Возможности

- ✅ Добавление новых задач
- ❌ Удаление существующих задач
- ✔️ Отметка выполнения задач
- 📋 Просмотр списка задач
- 💾 Автоматическое сохранение в базу данных
- 🖥️ Консольный интерфейс

## 📦 Требования

- Rust 1.60+
- Cargo
- SQLite3 (для сборки)

## ⚙️ Установка

1. Клонируйте репозиторий:
```bash
git clone https://github.com/navigatorsvs1232009/simple_rust_todo.git
cd rust-todo-cli
```

2. Соберите проект:
```bash
cargo build --release
```

3. Запустите программу:
```bash
./target/release/rust-todo-cli.exe  # Windows
./target/release/rust-todo-cli      # Linux/macOS
```

## 🕹 Использование

Главное меню:
```
1. Add Todo      - Добавить задачу
2. Remove Todo   - Удалить задачу
3. Complete Todo - Отметить выполненной
4. List Todos    - Показать список
5. Exit          - Выход
```

Пример работы:
```text
Enter todo description: Купить молоко
Todo added!

ID  | Status | Description
----|--------|------------
1   |   ❌   | Купить молоко
```

## 🛠 Сборка для Windows

1. Установите целевой компонент:
```bash
rustup target add x86_64-pc-windows-msvc
```

2. Соберите с статической линковкой SQLite (добавьте в Cargo.toml):
```toml
[dependencies]
rusqlite = { version = "0.31.0", features = ["bundled"] }
```

3. Скомпилируйте:
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

## 📁 Структура проекта
```
.
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs     # Основная логика
    ├── todo.rs     # Модель данных
    ├── todos_db.rs # Работа с БД
    └── ui.rs       # Пользовательский интерфейс
```
