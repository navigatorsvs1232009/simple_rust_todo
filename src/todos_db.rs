use rusqlite::{Connection, params, Error};
use crate::todo::Todo;

pub struct TodoList {
    conn: Connection,
}

impl TodoList {
    // Инициализация базы данных
    pub fn new() -> Result<Self, Error> {
        let conn = Connection::open("todos.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    // Добавление задачи
    pub fn add(&mut self, description: &str) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO todos (description) VALUES (?1)",
            params![description],
        )?;
        Ok(())
    }

    // Удаление задачи
    pub fn remove(&mut self, id: i32) -> Result<(), Error> {
        self.conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
        Ok(())
    }

    // Отметка выполнения задачи
    pub fn complete(&mut self, id: i32) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE todos SET completed = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    // Получение списка задач
    pub fn list(&self) -> Result<Vec<Todo>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, description, completed FROM todos")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }
}
