use std::error::Error;
use std::fs;

pub fn run(config:Config) ->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path).expect("something went wrong"); //? just passes the error to the function caller
    let query = config.query;
    let result = search(&query,&content);



    println!("results: \n{:?}",result);
    Ok(())
}
pub struct Config{
    pub query:String,
    pub file_path:String
}

impl Config{
    pub fn build(args: &[String])-> Result<Config,&'static str>{
        if args.len()<3 {
           return Err("not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{query,file_path})
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive
pick three.";

        assert_eq!(vec!["safe, fast, productive"],search(query,contents))
    }
}

pub fn search<'a>(query:&str, contents:&'a str)-> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }

    result
}