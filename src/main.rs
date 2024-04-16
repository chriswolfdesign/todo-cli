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

fn print_todos(todos: &Vec<Todo>, curr: usize) {
    for (index, todo) in todos.iter().enumerate() {
        if index == curr {
            if todo.completed {
                println!("{}", todo.text.black().on_white().strikethrough());
            } else {
                println!("{}", todo.text.black().on_white());
            }
        } else {
            if todo.completed {
                println!("{}", todo.text.strikethrough());
            } else {
                println!("{}", todo.text);
            }
        }
    }
}

fn increment_curr(curr: usize, todo_len: usize) -> usize {
    if curr < todo_len - 1 {
        curr + 1
    } else {
        curr
    }
}

fn decrement_curr(curr: usize) -> usize {
    if curr == 0 {
        curr
    } else {
        curr - 1
    }
}

async fn delete_curr(curr: usize, todos: Vec<Todo>) -> usize {
    if todos.len() == 0 {
        return 0;
    }
    let url = format!("http://localhost:8000/todos/{}", todos[curr].id);

    let client = reqwest::Client::new();
    let _ = client.delete(url).send().await;

    decrement_curr(curr)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut curr = 0;
    loop {
        clear_terminal();

        let todos = get_todos().await?;
        let todo_len = todos.len();

        print_todos(&todos, curr);

        let term = Term::stdout();
        let key = term.read_key().unwrap();

        match key {
            Key::Char('j') => curr = increment_curr(curr, todo_len),
            Key::Char('k') => curr = decrement_curr(curr),
            Key::Char('d') => curr = delete_curr(curr, todos).await,
            _ => println!("WTF you talking about bro?"),
        }
    }
}
