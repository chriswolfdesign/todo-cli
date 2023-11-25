use crate::manager::TodoManager;

mod manager;
mod model;
mod test;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut manager = TodoManager::new(String::from("http://localhost:3000/todos"));
    manager.update_todos().await?;
    manager.print_todos();

    Ok(())
}
