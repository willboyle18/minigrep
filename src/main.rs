use std::env;
use std::fs;

fn main() {
    // Hello from my ThinkPad

    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    
    let config = Config::new(&args);
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read file");
    
    println!("With text: \n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone(); // Using clone for simplicity
        let file_path = args[2].clone();

        Config { query, file_path}
    }
}
