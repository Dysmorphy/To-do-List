use colored::{ColoredString, Colorize};

use crate::cli::Action;
use serde::{Deserialize, Serialize};
use std::fmt;

fn color_string(str: &str) -> ColoredString {
    let new_str = str.bright_yellow();
    new_str
}

enum RenderRequest {
    Added,
    Deleted,
    NotDeleted,
    Help,
    Error,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    desc: String,
    id: u32,
}

impl Task {
    pub fn new(desc: String, id: u32) -> Self {
        Task { desc, id }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = format!("{})", self.id);
        write!(f, "{} {}", color_string(&id), self.desc)
    }
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, desc: String) {
        let new_task = Task::new(desc, (self.tasks.len() + 1) as u32);
        self.tasks.push(new_task);
    }

    fn fix_ids(&mut self) {
        let mut count: u32 = 1;
        for task in &mut self.tasks {
            task.id = count;
            count += 1;
        }
    }

    pub fn delete_task(&mut self, search_id: u32) -> Result<Task, String> {
        let mut found_idx: Option<usize> = None;
        for (task_idx, task) in self.tasks.iter().enumerate() {
            if task.id == search_id {
                found_idx = Some(task_idx);
                break;
            }
        }

        match found_idx {
            Some(delete_idx) => {
                let removed_task = self.tasks.remove(delete_idx);
                self.fix_ids();
                Ok(removed_task)
            }
            None => Err("id not found".to_string()),
        }
    }

    pub fn render_list(&self) -> String {
        let init_str = "List of current tasks:\n".to_string();
        let mut list_str = color_string(&init_str).to_string();
        for task in &self.tasks {
            list_str.push_str(&format!("{}\n", task));
        }
        list_str
    }

    fn render(&self, request: RenderRequest) {
        match request {
            RenderRequest::Added => {
                println!("{}","Task successfully added\n".bright_green());
                println!("{}", self.render_list());
            }

            RenderRequest::Deleted => {
                println!("{}","Task successfully deleted\n".bright_green());
                println!("{}", self.render_list());
            }

            RenderRequest::NotDeleted => {
                println!("{}","Failed to delete the task\n".bright_red());
                println!("{}", self.render_list());
            }

            RenderRequest::Help => {
                const HELP_MESSAGE: &str = "Usage: todo <command> [arguments]
Commands:
  add <task>        Add a new task
  rm <id>           Remove a task by its ID
  list              Display all current tasks
  --help            Show this help message";
                println!("{}", HELP_MESSAGE);
            }

            RenderRequest::List => {
                println!("{}", self.render_list());
            }

            RenderRequest::Error => {
                println!("{}","Not a valid command. Try --help for more information".bright_red());
            }
        }
    }

    pub fn perform_action(&mut self, action: Action) {
        match action {
            Action::Add(task_desc) => {
                self.add_task(task_desc);
                self.render(RenderRequest::Added)
            }

            Action::Remove(id) => {
                let output = self.delete_task(id);

                if let Ok(_task) = output {
                    self.render(RenderRequest::Deleted);
                } else {
                    self.render(RenderRequest::NotDeleted);
                }
            }

            Action::Error => {
                self.render(RenderRequest::Error);
            }

            Action::Help => {
                self.render(RenderRequest::Help);
            }

            Action::List => {
                self.render(RenderRequest::List);
            }
        }
    }

    pub fn import(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    pub fn export(&self) -> &Vec<Task> {
        &self.tasks
    }
}
