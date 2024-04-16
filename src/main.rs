use reqwest::Error;
use serde::Deserialize;

fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

#[derive(Deserialize, Debug)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    clear_terminal();

    let request_url = "http://127.0.0.1:8000/todos";
    println!("{}", request_url);

    let response = reqwest::get(request_url).await?;
    let users: Vec<Todo> = response.json().await?;

    dbg!(users);

    return Ok(())

}
