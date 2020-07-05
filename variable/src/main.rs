/*
Understanding rust type system

Mutable : let mut a = 5 // a is mutable

*/

fn another_function(){
    println!("printing function");
}


fn main() {

    /*

    //
    let mut x = 5;
    println!("x is {}",x);
    x =6 ;
    println!("x is {}",x);
    
    //shadowing
    //1.
    let y = 7;
    let y = y + 2; // repeating the let x
    println!("y is {}",y);
    let y = y * 2;
    println!("y is {}",y);
    //2.
    let spaces = "     "; //mut introduces warning
    let spaces = spaces.len();
    println!("spaces  is {}",spaces);
    
    //Numeic operations 
    let z1 = 5;
    let z2 = 10.2;
    let z3 = 11;
    println!("z2  is {}",z2);
    let z2 = z1 as f64 + z2; //casting in rust
    println!("z2  is {}",z2);
    let z4 : f64 = z1 as f64/z3 as f64;
    println!("z4  is {}",z4);
    
    // tuple
    let tup1 : (i32,f64,u8) = (500,6.8787,67);
    let (x,y,z) = tup1; 
    //let x = tup1.0;
    */
    another_function();
    
}
