use super::model::{TodoItem, TodoList};

pub struct TodoManager {
    url: String,
    client: reqwest::Client,
    todos: Vec<TodoItem>,
}

impl TodoManager {
    pub fn new(url: String) -> TodoManager {
        TodoManager {
            url,
            client: reqwest::Client::new(),
            todos: vec![],
        }
    }

    pub async fn update_todos(&mut self) -> Result<(), String> {
        let response = self.client.get(&self.url).send().await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<TodoList>().await {
                Ok(parsed) => self.todos = parsed.todos,
                Err(_) => return Err(String::from("could not parse")),
            },
            _ => return Err(String::from("incorrect status returned")),
        }

        Ok(())
    }

    pub fn print_todos(&self) {
        for todo in self.todos.iter() {
            println!("{}", todo.text);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::TodoManager;
    use crate::model::{TodoItem, TodoList};
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_all_todos_success() {
        let server = MockServer::start();

        let mut expected_result = TodoList { todos: vec![] };

        expected_result.todos.push(TodoItem {
            id: 1,
            text: String::from("hello"),
            completed: false,
        });
        expected_result.todos.push(TodoItem {
            id: 2,
            text: String::from("goodbye"),
            completed: true,
        });

        server.mock(|when, then| {
            when.method(GET).path("/todos");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!(expected_result));
        });

        let mut manager = TodoManager::new(server.url("/todos"));

        let response = manager.update_todos().await;

        match response {
            Ok(_) => {
                assert!(true);
            }
            _ => {
                assert!(false, "Error should not have occurred");
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_todos_404() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET).path("/todos");
            then.status(404);
        });

        let mut manager = TodoManager::new(server.url("/todos"));

        let response = manager.update_todos().await;

        match response {
            Ok(_) => {
                assert!(false, "Error should have occurred")
            }
            _ => {
                assert!(true)
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_todos_unexpected_json() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET).path("/todos");
            then.status(200).json_body(json!({ "foo": "bar" }));
        });

        let mut manager = TodoManager::new(server.url("/todos"));

        let response = manager.update_todos().await;

        match response {
            Ok(_) => {
                assert!(false, "Error should have occurred")
            }
            _ => {
                assert!(true)
            }
        }
    }
}
