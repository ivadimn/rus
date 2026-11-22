#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]


use std::fmt::Display;

fn print_pair<T: Display>(a: T, b: T) {
    // ваш код здесь
    println!("{},{}", a, b);
}

fn largest<T: PartialOrd + Ord>(list: &[T]) -> &T {
    // ваш код здесь
    list.iter().max().unwrap()
}

#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello world!");
    };
}

#[macro_export]
macro_rules! say {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

macro_rules! log {
    ($LEVEL:expr, $msg:expr ) => {
       println!("[{}] {}", $LEVEL, $msg);
    };
}

fn main() {
    print_pair(1, 2);             // → 1, 2
    print_pair("hello", "world"); // → hello, world

    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers)); // → 100
    
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&chars));   // → y

    say_hello!();
    say!("Привет Rust macro");
    log!("DEBUG", "Ошибка");
}