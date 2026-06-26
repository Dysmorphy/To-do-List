use crate::tasks::Task;

use dirs;
use serde_json;
use std::fs;
use std::path;

pub struct DataHandler {
    file_path: path::PathBuf,
    dir_path: path::PathBuf,
}

impl DataHandler {
    pub fn new() -> Self {
        DataHandler {
            file_path: dirs::home_dir().unwrap().join(".todo/tasks.json"),
            dir_path: dirs::home_dir().unwrap().join(".todo"),
        }
    }

    fn convert(tasks: &[Task]) -> String {
        serde_json::to_string(tasks).unwrap()
    }

    fn convert_back(converted: &String) -> Vec<Task> {
        let tasks: Vec<Task> = serde_json::from_str(&converted).unwrap();
        tasks
    }

    pub fn write(&self, tasks: &[Task]) {
        let converted = DataHandler::convert(tasks);
        let _ = fs::create_dir_all(&self.dir_path).unwrap();
        let _ = fs::write(&self.file_path, converted).unwrap();
    }

    pub fn read(&self) -> Vec<Task> {
        let content = fs::read_to_string(&self.file_path);
        if let Ok(converted) = content {
            DataHandler::convert_back(&converted)
        } else {
            Vec::new()
        }
    }
}
