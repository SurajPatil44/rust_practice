use std::f64;

struct Point {
x: f64,
y: f64
}

impl Point{

    fn eucl_distance(&self,point2 : &Point)-> f64{
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
   
    fn angle(&self,other : &Point)-> f64{
    /*
                * (x2,y2)   
               /|                    abs(x2-x1)
              / |     angle = asin(  ----------       )
             /  |                    eucl_distance()
            *
          (x1,y1)
    */
    let result = (self.x - other.x).abs() / self.eucl_distance(other) as f64;
    result.asin()
    
    }
    
    fn origin()-> Point{
        Point{
            x:0 as f64,
            y:0 as f64,
        }      
    }
    
}


fn main() {
    
    let point1 = Point::origin();
    let x = 2.0;
    let y = 3.0;
    //let point1 = Point{x:x,y:y};
    let point2 = Point{x:2.0,y:2.0};
    println!("{}",point1.angle(&point2));
    
    /*
    for i in 1..1001{
        let x = x + (i%5) as f64;
        let y = y + (i%5) as f64;
        let point2 = Point{x,y};
        let index = i;
        //println!("{} {} {}",index,point1.eucl_distance(&point2),point1.angle(&point2));
    }
    */
}
