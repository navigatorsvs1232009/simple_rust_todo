pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i32, description: String, completed: bool) -> Self {
        Self { id, description, completed }
    }
}
