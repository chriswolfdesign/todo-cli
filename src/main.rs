use colored::Colorize;
use reqwest::Error;
use serde::Deserialize;

use console::{Key, Term};

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
    let mut curr = 0;
    loop {
        clear_terminal();

        let todos = get_todos().await?;
        let todo_len = todos.len();

        print_todos(todos, curr);

        let term = Term::stdout();
        let key = term.read_key().unwrap();

        if key == Key::Char('j') {
            if curr < todo_len - 1 {
                curr += 1;
            }
        } else if key == Key::Char('k') {
            if curr > 0 {
                curr -= 1;
            }
        }
    }
}
