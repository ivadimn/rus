use std::ops::Add;
mod jsn;
use jsn::{Person, struct_to_bytes};

struct Complex {
    real: f64,
    img: f64,
}

impl Add for Complex {

    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            real: self.real + rhs.real,
            img: self.img + rhs.img,
        }
    }


}


fn main() {

    let p = Person {
        name: String::from("Пупкин"),
        age: 77,
    };

    let ser = serde_json::to_string(&p).unwrap();
    println!("Data {}", ser);

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let squares = numbers.iter().map(|&x| x * x).collect::<Vec<_>>();
    for sq in squares {
        println!("{}", sq);
    } 

    let c1 = Complex {real: 2.45, img: 0.34};
    let c2 = Complex {real: 1.26, img: 0.12};
    let c3 = c1 + c2;
    println!("Complex {} + {}i", c3.real, c3.img);
}
