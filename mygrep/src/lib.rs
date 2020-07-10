use std::fs;
use std::error::Error;

pub struct Config{
    fname : String,
    pattern : String
}

pub struct SearchInfo{
    lineno : u32,
    line : String
}

impl Config{
    pub fn new(args: &[String]) -> Result<Self,&'static str>{
        let file = args[1].clone();
        let pattern = args[2].clone();
        
        Ok(Config{fname:file,pattern:pattern})
    }
}

pub fn run(config : Config) -> Result<(),Box<dyn Error>>{
    
    let contents = fs::read_to_string(config.fname)?;
    //&contents
    for si in search(&contents,&config.pattern,true){
        eprintln!("{}. {}",si.lineno,si.line);
    }
    Ok(())
}

pub fn search<'a> (content : &'a str,query : &str,
                    case_insensitive : bool) -> Vec<SearchInfo>
{
    let mut results = Vec::new();
    match case_insensitive{
        true => {
                    let query = query.to_lowercase();
    
                    for (index,line) in content.lines().enumerate(){
                        if line.to_lowercase().contains(&query){
                            let si = SearchInfo{
                                        lineno : index as u32,
                                        line : line.to_string()
                                    };
                            results.push(si);
                        }
                    } 
                },
        false =>{
                    for (index,line) in content.lines().enumerate(){
                        if line.contains(query){
                            let si = SearchInfo{
                                        lineno : index as u32,
                                        line : line.to_string()
                                    };
                            results.push(si);
                        }
                    } 
                }
    }
    results
            
} 



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], 
                    search(contents,query,false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(contents,query,true)
        );
    }
}