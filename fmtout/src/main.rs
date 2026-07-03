use std::fmt;


#[derive(Debug)]
struct MinMax(i64, i64);

struct Point2D {
    x: f64,
    y: f64
}


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x = {}, y = {}", self.x, self.y)
    }
}

fn main() {
    
    let minmax = MinMax(8, 54);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let p = Point2D {x: 2.6, y: 3.5};
    println!("Point 2D {}", p);
}






