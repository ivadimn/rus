use std::io;
//use core::ops::AddAssign;
mod helpers;
use helpers::*; 

static PROGRAM_NAME: &'static str = "Super App";
static mut REQUEST_COUNT: u32 = 0;

fn main() {
  let mut s = String::from("Hello");

    // Конкатенация
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Обратите внимание: s1 перемещается и больше недоступна

    // Форматирование
    let formatted = format!("{}, {}!", "Hello", "world"); // Более гибкий способ

    // Срезы и индексация
    let hello = &s3[0..5];

    // Итерация по символам
    for c in s3.chars() {
        println!("{}", c);
    }

    // Итерация по байтам
    for b in s3.bytes() {
        println!("{}", b);
    }
}

#[allow(dead_code)]
fn test_if() {
    let age_to_drive = 16u8;
    println!("Enter persons age: ");

    let myinput: &mut String = &mut String::from("");
    io::stdin().read_line(myinput).unwrap();
    let age  = myinput.trim().parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing driver's because they are old enough!");
    }
    // let myinput: u8 = match myinput.trim().parse() {
    //     Ok(num) => num,
    //     Err(err) => println!("Error: {}", msg),
    // };
}

fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    let abs_diff = (a - b).abs();
    
    if a == b {  // Точное равенство для одинаковых чисел
        true
    } else if a == 0.0 || b == 0.0 || abs_diff < f64::MIN_POSITIVE {
        // Особые случаи при работе с нулём или очень маленькими числами
        abs_diff < (epsilon * f64::MIN_POSITIVE)
    } else {
        // Общий случай - используем относительную погрешность
        abs_diff / (a.abs() + b.abs()) < epsilon
    }
}

 