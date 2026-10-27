#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

struct Rectangle {
    width: f64,
    height: f64,
}

fn area(rect: Rectangle) -> f64 {
    rect.width * rect.height
}

fn perimeter(rect: Rectangle) -> f64 {
    (rect.width + rect.height) * 2.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn swap_tuple(pair: (i32, i32)) -> (i32, i32) {
    (pair.1, pair.0)
}

fn main() {
    let rect = Rectangle {
        width: 10.0, 
        height: 5.0,
    };
    println!("Площадь: {}", area(rect));
    let rect = Rectangle {
        width: 10.0, 
        height: 5.0,
    };
    println!("Периметр: {}", perimeter(rect));
    println!("22 C = {} F", celsius_to_fahrenheit(22.0));
    println!("71.6 F = {:.1} C", fahrenheit_to_celsius(71.6));
    println!("swap = {:?}", swap_tuple((1, 2)));
}