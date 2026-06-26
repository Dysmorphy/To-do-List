mod tasks;
mod cli;
mod storage;

fn main() {
    let action = cli::parse_args();   
    let mut task_list = tasks::TaskManager::new();
    let storage = storage::DataHandler::new();
    
    let read_data = storage.read();
    task_list.import(read_data);
    task_list.perform_action(action);
    let current_state = task_list.export();
    storage.write(current_state);
}
