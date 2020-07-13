use std::fs;
use std::error::Error;

pub struct Config{
    fname : String,
    pattern : String,
    case_insensitive : bool
}

#[derive(Debug,Eq,PartialEq)]
pub struct SearchInfo{
    lineno : u32,
    line : String
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Self,&'static str>{
        args.next(); //filename
        /*
        let file = args[1].clone();
        let pattern = args[2].clone();
        */
        let file = match args.next(){
            Some(arg) => arg,
            None => return Err("Filename is not provided")
        };
        let pattern = match args.next(){
            Some(arg) => arg,
            None => return Err("Query pattern is not provided")
        };
        
        
        let case_insensitive = match args.next(){
            Some(arg) => {
                let arg = String::from(arg);
                if arg == "0"{
                    false
                }else{
                    true
                }
            },
            None => true
        };
        
        Ok(Config{fname:file,
               pattern,
               case_insensitive
        })
    }
}

pub fn run(config : Config) -> Result<(),Box<dyn Error>>{
    
    let contents = fs::read_to_string(config.fname)?;
    //&contents
    for si in search(&contents,&config.pattern,config.case_insensitive){
        eprintln!("{}. {}",si.lineno,si.line);
    }
    Ok(())
}

pub fn search<'a> (content : &'a str,query : &str,
                    case_insensitive : bool) -> Vec<SearchInfo>
{
    
    let mut results = Vec::new();
    let cont_iter = content
                    .lines()
                    .enumerate();
    match case_insensitive{
        true => {
                    let query = query.to_lowercase();
                    results = cont_iter
                                .map(|(i,line)| SearchInfo{lineno: (i + 1) as u32,line:line.to_string()})
                                .filter( |si| si.line.to_lowercase().contains(&query))
                                .collect();              
                    
                },
        false =>{ 
                    results = cont_iter
                                .map(|(i,line)| SearchInfo{lineno: (i + 1) as u32,line:line.to_string()})
                                .filter( |si| si.line.contains(&query))
                                .collect(); 
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

        let answer = search(contents,query,false);
        /*
        for ans in answer{
            println!("{} {}",ans.lineno,ans.line);
        }
        */
        assert_eq!(vec![
                       SearchInfo{
                        lineno : 2,
                        line: String::from("safe, fast, productive."),
                       }
                    ], 
                    answer
                   );
        
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let answer = search(contents,query,true);
        /*
        for ans in answer{
            println!("{} {}",ans.lineno,ans.line);
        }
        */
        assert_eq!(vec![
                       SearchInfo{
                        lineno : 1,
                        line: String::from("Rust:"),
                       },
                       SearchInfo{
                        lineno : 4,
                        line: String::from("Trust me."),
                       },
                    ], 
                    answer
                   );
        
    }
}