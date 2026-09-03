use std::{fmt::Debug, ops::Add, ops::Sub};
use std::io::BufRead;
mod jsn;
use jsn::{Person, struct_to_bytes};
mod serr;
use serr::MyError;
mod wp;
use wp::*;

trait Drawable {
    fn draw(&self);
}

trait Printable {
    fn print(&self);
}

trait Displayable : Printable {
    fn display(&self);
}

impl Printable for Person {
    fn print(&self) {
        println!("Name {}", self.name);
    }
}

impl Displayable for Person {
    fn display(&self) {
        println!("Выводится на экран: {}", self.name);
    }
}

#[derive(Debug)]
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

fn find_user(username: &str) -> Result<u32, String> {

    let f = std::fs::File::open("/etc/passwd")
        .map_err(|e| format!("Faild to open password file: {:?}", e))?;
    Ok(45)
}

const MAX_LEN: usize = 1024;

fn first_line(filename: &str) -> Result<String, MyError> {
    let file = std::fs::File::open(filename)?;
    let mut reader = std::io::BufReader::new(file);
    let mut buf = vec![];
    let len = reader.read_until(b'\n', &mut buf)?;
    let result = String::from_utf8(buf).map_err(MyError::Utf8)?;
    if result.len() > MAX_LEN {
        return Err(MyError::General(format!("Line too long: {}", len)));
    }
    Ok(result)
}

#[derive(Copy, Clone, Debug)]
struct MyStruct;

trait Unit: Default + Copy {
    fn unit(&self) ->&'static str;
}

#[derive(Default, Copy, Clone, Debug)]
struct Meter;

impl Unit for Meter {
    fn unit(&self) ->&'static str {
        "м"
    }   
}

#[derive(Default, Copy, Clone)]
struct Kilogram;

impl Unit for Kilogram {
    fn unit(&self) ->&'static str {
        "кг"
    }
}

#[derive(Debug)]
struct Quantity<U: Unit> {
    value: f64,
    unit: U,
}

impl<U: Unit> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            unit: U::default(),
        }
    }
}

impl<U: Unit> Sub for Quantity<U>{
    type Output = Quantity<U>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            unit: U::default(),
        }
    }    
}

trait Shape {
    fn area(&self) -> f64;
}


struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
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

    if let Ok(id) = find_user("vadim") {
        println!("User Id: {}", id);
    }
    
    if let Ok(line) = first_line("output.txt") {
        println!("{}", line);
    }
    else {
        println!("Error!");
    }

    let original = MyStruct;
    let copy = original;
    let clone = original.clone();
    
    println!("Original {:?}", original);
    println!("Copy {:?}", copy);
    println!("Clone {:?}", clone);

    p.print();
    p.display();

    let dist1 = Quantity::<Meter> {value: 5.0, unit: Meter};
    let dist2 = Quantity::<Meter> {value: 3.0, unit: Meter};
    let total_dist = dist1 + dist2;
    println!("Общее расстояние: {:?}", total_dist);

    let c: &dyn Shape = &Circle {radius: 4.0};
    println!("Площадь окружности {}", c.area());

    wp::attack::<wp::Warrior>();
    wp::attack::<wp::Mage>();


}
