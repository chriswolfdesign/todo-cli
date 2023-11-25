use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
}
