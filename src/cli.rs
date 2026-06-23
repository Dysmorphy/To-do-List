use std::env;

enum Action {
    Add(String),
    Remove(u32),
    Help,
    Error
}

pub fn read_args() -> Vec<String> {
    let args:Vec<String> = env::args().collect();
    let clean_args = args[2..].to_vec();
    clean_args
}

pub fn parse_args() -> Action {
    let args = read_args();
    match args.len() {
        1 => {
            let arg = args[0].as_str(); 
            match arg {
                "--help" => Action::Help ,
                _ => Action::Error
            }
            
        },

        2 => {
            let first_arg = args[0].as_str();
            let second_arg = args[1].clone();
            match first_arg {
                "add" => Action::Add(second_arg), 
                "remove" => {
                    let parsed_arg = second_arg.parse::<u32>();
                    if let Ok(id) = parsed_arg {
                        Action::Remove(id)
                    } else {
                        Action::Error
                    }

                },
                _ => Action::Error

            }

        }

        _ => Action::Error
    }

}
