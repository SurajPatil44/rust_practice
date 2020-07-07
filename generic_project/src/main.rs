//Generic Coding in Rust
//1. Numerical types : 
//   A. add numtraits PrimeInt and Floats
//   B. add traits from Numtraits
//   C. std::ops::{Add,Sub,Mul,DIv}
//   D. use if let 
//
//
//
//
//
//
//
//
extern crate num_traits;

use num_traits::ToPrimitive;

struct SomeType<T>{
    x:T,
    y:T
}

enum Angle{
    inRad : f64,
    inDeg : f64
}


impl<T: ToPrimitive+Copy+Clone> SomeType<T>{

    fn new(x : T, y: T) -> Option<SomeType<f64>> {
        let new : SomeType<T> = SomeType{ x : x, y : y};
        new.as_float()
    }

    fn as_float(&self)-> Option<SomeType<f64>>{
        let x = self.x.to_f64()?;
        let y = self.y.to_f64()?;
        Some(SomeType{x,y})  
    }
}

impl SomeType<f64>{

    fn eucl_distance(&self,point2 : &SomeType<f64>)-> f64{
        // dist = sqrt((x1 - x2)**2 + (y1 - y2)**2))
        let result = (self.x - point2.x)*(self.x - point2.x) +
                      (self.y - point2.y)*(self.y - point2.y);
        result.sqrt()
    }
    
    fn mag(&self) -> f64{
        //sqrt(x**2+y**2)
        let res = (self.x * self.x) + (self.y * self.y);
        res.sqrt()
    }
   
    fn angle(&self,other : &SomeType<f64>)-> f64 {
    /*
                * (x2,y2)   
               /|                    abs(x2-x1)
              / |     angle = asin(  ----------   )
             /  |                    abs(y2-y1)
            *
          (x1,y1)
    */
    let result = (self.x - other.x).abs() / (self.y - other.y).abs();
    result.atan()
    
    }
    
    fn origin()-> SomeType<f64>{
        SomeType{
            x:0 as f64,
            y:0 as f64,
        }      
    }
}


fn main() {
    let s  = SomeType::new(3.2,4.2).unwrap();
    let p  = SomeType::new(3,4).unwrap();
    let r  = SomeType::new(4,4).unwrap();
    let o  = SomeType::origin();
    
    assert_eq!(s.eucl_distance(&p) == p.eucl_distance(&s),true);
    assert_eq!(p.mag() == 5_f64,true);
    println!("Euclidean distance {}",r.angle(&o));
    
    /*
    let s  = match s{
                Some(s) => s,
                _ => {} ,
    };
    let p  = match p{
                Some(p) => p,
                _ => {},
    };
    */
}
