use std::{env,process};

use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file: {}",config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
        //get familiar with this error handling syntax
        //we didn't use the unwrap_or_else syntax because run() doesn't return a meaningful value
        //to unwrap and it's better to do it this way instead
    }
}



