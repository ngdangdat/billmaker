use toml::Table;
use std::env;
use std::fs;




fn parse_config(config_file: &str) {
    let raw = fs::read_to_string(config_file).unwrap();
    let parsed = raw.parse::<Table>().unwrap();
}


fn main() {
    let mut config_file: String = "billing.toml".to_string();
    let mut task_file: String = "tasks.md".to_string();
    if let Some(file_arg) = env::args().nth(1) {
        config_file = file_arg.to_string();
    }
    if let Some(file_arg) = env::args().nth(2) {
        task_file = file_arg.to_string();
    }

    println!("config_file: {}, task_file: {}", config_file, task_file);
    parse_config(&config_file);
}

