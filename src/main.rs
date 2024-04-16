use reqwest::Error;
use serde::Deserialize;
use colored::Colorize;

#[derive(Deserialize, Debug)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

async fn get_todos() -> Result<Vec<Todo>, Error> {
    let request_url = "http://127.0.0.1:8000/todos";

    let response = reqwest::get(request_url).await?;
    let todos = response.json().await?;

    Ok(todos)
}

fn print_todos(todos: Vec<Todo>, curr: usize) {
    for (index, todo) in todos.iter().enumerate() {
        if index == curr {
            println!("{}", todo.text.black().on_white());
        } else {
            println!("{}", todo.text);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    clear_terminal();

    let todos = get_todos().await?;

    let curr = 0;

    print_todos(todos, curr);

    return Ok(());
}
