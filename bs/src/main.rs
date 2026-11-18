#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
use std::io::{self, Write};

use crate::storage::Storage;
mod balance;
mod storage;
mod error;

fn help_print() {
    println!("\nВведите команду:");
    println!("add <username>              - добавить пользователя.");
    println!("deposit <username> <summa>  - положить деньги.");
    println!("withdraw <username> <summa> - снять деньги.");
    println!("balance <username>          - .");
    println!("list                        - показать список пользователей.");
    println!("exit                        - закрыть программу.");
    //println!("add <username> - добавить пользователя.");

    
}

fn main() {

    let filename = "bank.csv";
    println!("Программа обслуживания банковских счетов.");
    let mut storage = Storage::load_data(filename);
    loop {
        help_print();
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or(0);
        if input.len() == 0 {
            continue;
        }
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        match args[0] {
            "add" => {
                if args.len() != 3 {
                    println!("Пример использования: add Alice 1000");
                    continue;
                }
                let name = args[1].to_string();
                
            },
            "deposit" => {},
            "withdraw" => {},
            "balance" => {},
            "list" => {},
            "exit" => {},
            _ => continue,
        }
                
        break;
    }
    
    println!("Hello, world!");
}
