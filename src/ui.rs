pub fn print_todos(todos: &[super::todo::Todo]) {
    if todos.is_empty() {
        println!("No todos found!");
        return;
    }

    println!("\nID  | Статус | Описание");
    println!("----|--------|------------");
    for todo in todos {
        let status = if todo.completed { "✔️" } else { "❌" };
        println!("{:3} | {:^6} | {}", todo.id, status, todo.description);
    }
}
