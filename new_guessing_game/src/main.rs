use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number OK!");
    
    let secret_num: u8 = rand::thread_rng().gen_range(1,101);
    
    println!("The secret number is: {}", secret_num);
    
    loop{
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
        
        /*
        Switching from an expect call to a match expression is how you generally move from crashing on an error to handling the error. Remember that parse returns a Result type and Result is an enum that has the variants Ok or Err. We’re using a match expression here, as we did with the Ordering result of the cmp method
        */
        
        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
              
        println!("You guessed : {}",guess);
        
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
