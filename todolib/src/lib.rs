use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct TodoRecord {
    pub id: Uuid,
    pub name: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoStore {
    pub records: Vec<TodoRecord>,
}

impl TodoStore {
    pub fn new() -> TodoStore {
        TodoStore { records: vec![] }
    }
    pub fn add(&self, name: String) {}
    pub fn mark_as(&self, id: Uuid, is_done: bool) {}
    pub fn mark_all_as(&self, is_done: bool) {}
    pub fn remove(&self, id: Uuid) {}
    pub fn remove_all_done(&self) {}
}

fn watch_todo_records(folder_path: String) -> notify::Result<()> {
    match fs::create_dir_all(folder_path.clone()) {
        Ok(_) => println!("Directory created or already exists."),
        Err(e) => println!("Error creating directory: {}", e),
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder_path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                log::info!("Change: {event:?}");
            }
            Err(error) => {
                log::error!("Error: {error:?}");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::watch_todo_records;

    #[test]
    fn watch_todo_record_test() {
        watch_todo_records("/tmp/todo_record_test".to_string())
    }

    #[test]
    fn add_record_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
