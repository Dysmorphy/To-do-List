use colored::Colorize;

use crate::cli::Action;
use std::fmt;

enum RenderRequest {
    Added,
    Deleted,
    NotDeleted,
    Help,
    Error,
    List
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Task {
    desc:String,
    id:u32,
}

impl Task {
    pub fn new(desc:String,id:u32) -> Self {
        Task {
            desc,
            id
        }
    }

}

impl fmt::Display for Task {
    fn fmt(&self,f:&mut fmt::Formatter <'_>) -> fmt::Result {
        let id = format!("{})",self.id).bold().yellow();
        write!(f,"{} {}",id,self.desc)
    }
}

#[derive(Debug)]
pub struct TaskManager {
    tasks:Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks:Vec::new(),
        }
    }
    
    pub fn add_task(&mut self,desc:String) {
        let new_task = Task::new(desc,self.tasks.len() as u32);
        self.tasks.push(new_task);
        
    }

    fn fix_ids(&mut self) {
        let mut count:u32 = 0;
        for task in &mut self.tasks {
            task.id = count;
            count+=1;
        }
    }

    pub fn delete_task(&mut self,search_id:u32) -> Result<Task,String> {
        let mut found_idx:Option<usize> = None;
        for (task_idx,task) in self.tasks.iter().enumerate() {
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
            None => Err("id not found".to_string())
        }
    }
    
    fn render_list (&self) -> String {
        "132".to_string()
    }
    pub fn render(&self,request:RenderRequest) {
        match request {
            RenderRequest::Added => {
                //planning to print a message then show the whole list using the format_list 
            },

            _ => ()



        }
    }

    pub fn perform_action(&mut self,action:Action) {
        match action {
            Action::Add(task_desc) => {
                self.add_task(task_desc);
                self.render(RenderRequest::Added)
            },

            Action::Remove(id) => {
                let output = self.delete_task(id);

                if let Ok(_task) = output {
                    self.render(RenderRequest::Deleted);
                } else {
                    self.render(RenderRequest::NotDeleted);
                }
            },

            Action::Error => {
                self.render(RenderRequest::Error);
            },

            Action::Help => {
                self.render(RenderRequest::Help);
            }

            Action::List => {
                self.render(RenderRequest::List);
            }
        }
    }
    
}
