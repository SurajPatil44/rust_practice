use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use serde::Deserialize;

fn as_u32_le(array : &[u8;4]) -> u32 {
    ((array[0] as u32) << 0) +
    ((array[1] as u32) << 8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24) 
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct FileData{
    Name: String,
    Type: String,
    Size: i32,
}

impl FileData{
    fn print_data(&self){
       println!("{} {} {}", self.Name,self.Type,self.Size);
    }
}

fn quit(mut stream: &TcpStream){
    
    let mut buffer = [0; 4];
    let query = String::from("quit");
    stream.write(query.as_bytes()).unwrap();
    stream.read(&mut buffer).unwrap();
    let num = as_u32_le(&buffer);
    let mut buffer2: Vec<u8> = vec![0; num as usize];
    stream.read(&mut buffer2).unwrap();
    println!("data: {}", String::from_utf8_lossy(&buffer2[..]));
}
    
fn main() {
    let stream = TcpStream::connect("127.0.0.1:5559").unwrap();
    //enum choices = (u8
    loop{
        let mut choice = String::new();
        println!("\nchoose from 1,2,3\n");
        io::stdin()
          .read_line(&mut choice)
          .expect("FAILED!!!");
          
        
        let choice: u8 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match choice{
            1 => handle_get_data(& stream),
            _ => {
                quit(& stream);
                break;
            
            }
        }
    }
          
}

fn handle_get_data(mut stream: &TcpStream){

    let mut buffer = [0; 4];
    let query = String::from("get_data");
    
    stream.write(query.as_bytes()).unwrap();

    stream.read(&mut buffer).unwrap();
    
    let num = as_u32_le(&buffer);
    
    //println!("Request: {}", num);
    
    for _ in 0..num{
        let mut buffer = [0; 4];
        stream.read(&mut buffer).unwrap();
        let num = as_u32_le(&buffer);
        //println!("size: {}", num);
        let mut buffer2: Vec<u8> = vec![0; num as usize];
        stream.read(&mut buffer2).unwrap();
        let the_file = String::from_utf8_lossy(&buffer2[..]);
        let file_dat: FileData = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
        file_dat.print_data();
    }

}

