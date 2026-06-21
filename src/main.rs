#[derive(Debug)]
struct Task {
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
struct TaskManager {
    tasks:Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks:Vec::new(),
        }
    }
    
    fn add_task(&mut self,desc:String) {
        let new_task = Task::new(desc,self.tasks.len() as u32);
        self.tasks.push(new_task);
        
    }

    fn fix_ids(&mut self) {
        let count:u32 = 0;
        for task in &mut self.tasks {
            task.id = count;
        }
    }

    fn delete_task(&mut self,search_id:u32) {
        let mut found = false;
        for (task_idx,task) in self.tasks.iter().enumerate() {
            if task.id == search_id {
                self.tasks.remove(task_idx);
                found = true;
                break;
            }
        }

        if found == false {
         //figure out error messaging   
        }
        self.fix_ids();
    }
    
}

fn main() {
    let mut test = TaskManager::new();
    test.add_task("123".to_string());
    test.add_task("223".to_string());
    test.delete_task(0);
    println!("{:?}",test);
}
