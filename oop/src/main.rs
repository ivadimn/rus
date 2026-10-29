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

enum Command {
    Show(&'static str),
    Add(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
    Stop,
}

fn calculate(command: &Command) -> Option<i32> {
    match command  {
        Command::Add(a, b) => Some(a + b),
        Command::Multiply(a, b) => Some(a * b),
        Command::Divide(a, b) if *b != 0 => Some(a / b),
        _ => None,
    }
}

fn main() {
    let commands = vec![
        Command::Show("Стартуем"),
        Command::Divide(20, 4),
        Command::Divide(20, 0),
        Command::Multiply(5, 4),
        Command::Show("Финишная прямая"),
    ];    

    for command in &commands  {
        match command {
            Command::Show(msg) if let Some(first) = msg.chars().next() => {
                println!("Сообщение: {}. Первый символ: {}", msg, first);
            },
            Command::Stop => {
                println!("Останавливаем обработку команд");
                break;
            }
            _ => if let Some(result) = calculate(command) {
                println!("Результат: {}", result);
            } else {
                println!("Операция не выполнена.");
            },
        }
    }
}