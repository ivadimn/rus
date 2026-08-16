struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
} 

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Shape squre: {}", shape.area())
}

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.count)
        } else {
            None    
        } 
    }
}

use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize ,Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut stack = Stack::<i32>{items: Vec::new()};

    stack.push(10);
    stack.push(20);
    stack.push(30);

    while let Some(item) = stack.pop()  {
        println!("item: {}", item);
    }

    let default = Circle {radius: 2.5};
    let rect = Rectangle {width: 10., height: 15.};

    print_area(&default);
    print_area(&rect);

    let mut count = Counter{count: 10};
    while let Some(number) = count.next() {
        println!("Iteration: {}", number);
    }
    
    let person = Person {
        name: String::from("Vadim Ivanov"),
        age: 61,
    };

    let person_string = serde_json::to_string(&person).unwrap();
    println!("Person: {}", person_string);

}
