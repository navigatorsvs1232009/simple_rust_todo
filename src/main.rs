mod todo;
mod todos_db;
mod ui;

use std::io::{self, Write};
use crate::{todos_db::TodoList, ui::print_todos};

fn main() {
    let mut todo_list = match TodoList::new() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error initializing database: {}", e);
            return;
        }
    };

    loop {
        // Меню управления
        println!("\nСписок дел");
        println!("1. Добавить дело");
        println!("2. Удалить дело");
        println!("3. Завершить дело");
        println!("4. Список дел");
        println!("5. Выход");

        // Обработка ввода пользователя
        print!("Выберите пункт меню: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Выбор действия
        match input.trim() {
            "1" => {
                print!("Добавьте описание: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                if let Err(e) = todo_list.add(description.trim()) {
                    eprintln!("Error adding todo: {}", e);
                } else {
                    println!("Todo added!");
                }
            }
            "2" => {
                match todo_list.list() {
                    Ok(todos) => print_todos(&todos),
                    Err(e) => eprintln!("Error listing todos: {}", e),
                }

                print!("Введите номер дела который хотите удалить: ");
                io::stdout().flush().unwrap();

                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();

                match id_input.trim().parse::<i32>() {
                    Ok(id) => {
                        match todo_list.remove(id) {
                            Ok(()) => println!("Todo removed!"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    _ => println!("Invalid ID!"),
                }
            }
            "3" => {
                match todo_list.list() {
                    Ok(todos) => print_todos(&todos),
                    Err(e) => eprintln!("Error listing todos: {}", e),
                }

                print!("Enter the ID of the todo to complete: ");
                io::stdout().flush().unwrap();

                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();

                match id_input.trim().parse::<i32>() {
                    Ok(id) => {
                        match todo_list.complete(id) {
                            Ok(()) => println!("Todo marked as completed!"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    _ => println!("Invalid ID!"),
                }
            }
            "4" => {
                match todo_list.list() {
                    Ok(todos) => print_todos(&todos),
                    Err(e) => eprintln!("Error listing todos: {}", e),
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
