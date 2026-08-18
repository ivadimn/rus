use std::io;
//use core::ops::AddAssign;
mod helpers;
use helpers::*; 

static PROGRAM_NAME: &'static str = "Super App";
static mut REQUEST_COUNT: u32 = 0;

fn main() {
    let x = 0.1 + 0.2;
    let y = 0.3;
    
    // Разные способы сравнения
    println!("Прямое сравнение (неправильно):");
    println!("  x == y: {}", x == y);
    
    let delta: f64 = x - y;
    println!("\nСравнение с epsilon:");
    println!("  |x - y| <= epsilon: {}", delta.abs() <= f64::EPSILON);
    
    println!("\nОтносительное сравнение:");
    println!("  approx_eq(x, y, 1e-14): {}", approx_eq(x, y, 1e-14));
    
    // Демонстрация работы с разными значениями
    let test_cases = [
        (1.0, 1.0 + f64::EPSILON),
        (1000000.0, 1000000.0 + 0.0001),
        (0.0, 1e-17),
        (f64::MIN_POSITIVE, f64::MIN_POSITIVE * 1.1)
    ];
    
    println!("\nРазличные тестовые случаи:");
    for (a, b) in test_cases.iter() {
        println!("  Сравнение {} и {}:", a, b);
        println!("    approx_eq: {}", approx_eq(*a, *b, 1e-14));
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

 