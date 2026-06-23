#[derive(Debug)]
#[derive(Clone)]
pub struct Task {
    desc:String,
    id:u32,
}

impl Task {
    fn new(desc:String,id:u32) -> Self {
        Task {
            desc,
            id
        }
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
    
}
