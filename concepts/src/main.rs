//concepts

fn add(str2mod : &mut String, str2add: &str){
    str2mod.push_str(str2add)
    
    //can' return &str2mod directly 
}

fn find_first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    
    &s[..]
        
}

fn main() {
    
    /*
    Ownership rules
   
    1. Each value in Rust has a variable that’s called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.   
    
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1.push_str(", world!");
    
    
    println!("{}",s2);
    */
    //avoid data races
    /*
    Data race
    
    * Two or more pointers access the same data at the same time.
    * At least one of the pointers is being used to write to the data.
    * There’s no mechanism being used to synchronize access to the data.
    
    let mut s = String::from("");
    
    {
        let add_this = "Hello ";
        add(&mut s,add_this);
        
        //without scope can't do this
    }
    
    let add_this = ",world !";
    add(&mut s,add_this);
    
    println!("{}",s);
    */
    let  _str = String::from("Two or more pointers access the same data at the same time.");
    let siz = find_first_word(&_str);  
    println!("{}",siz); 
    
}
