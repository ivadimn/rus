#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
use std::io::{self, Write};

mod balance;
mod storage;
mod error;
mod transaction;
use crate::storage::Storage;
use crate::balance::Balance;

fn help_print() {
    println!("\nВведите команду:");
    println!("add <username> <summa>      - добавить пользователя.");
    println!("deposit <username> <summa>  - положить деньги.");
    println!("withdraw <username> <summa> - снять деньги.");
    println!("balance <username>          - показать баланс пользователя.");
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
                let Ok(balance) = args[2].parse::<i64>() else {
                    println!("Не правильно введено сумма!");
                    continue;
                };
                if let Err(err) = storage.add_user(name.clone()) {
                    println!("Ошибка добавления пользователя: {}", err);
                    continue;
                }
                if let Err(err) = storage.deposit(&name, Balance(balance)) {
                    println!("Ошибка изменения баланса: {}", err);
                    continue;
                }
                println!("Пользователе с именем {} добпавлен текущзий баланс: {}", name, balance);
                storage.save(filename);
            },
            "deposit" => {
                if args.len() != 3 {
                    println!("Пример использования: deposit Alice 1000");
                    continue;
                }
                let name = args[1].to_string();
                let Ok(balance) = args[2].parse::<i64>() else {
                    println!("Не правильно введена сумма!");
                    continue;
                };

                match storage.deposit(&name, Balance(balance)) {
                    Ok(balance) => { 
                        println!("Баланс пользователе с именем {} увеличен текущзий баланс: {}", name, balance);
                        storage.save(filename);
                    },
                    Err(err) => println!("Ошибка увеличения баланса: {}", err),
                }
            },
            "withdraw" => {
                if args.len() != 3 {
                    println!("Пример использования: withdraw Alice 1000");
                    continue;
                }
                let name = args[1].to_string();
                let Ok(balance) = args[2].parse::<i64>() else {
                    println!("Не правильно введена сумма!");
                    continue;
                };

                match storage.withdraw(&name, Balance(balance)) {
                    Ok(balance) =>  {
                        println!("Баланс пользователе {} уменьшен текущзий баланс: {}", name, balance);
                        storage.save(filename);
                    },
                    Err(err) => println!("Ошибка изменения баланса: {}", err),
                }
            },
            "balance" => {
                if args.len() != 2 {
                    println!("Пример использования: balance Alice");
                    continue;
                }
                let name = args[1].to_string();
                match storage.get_balance(&name) {
                    Ok(balance) => println!("Текущий баланс пользователе {}: {}", name, balance),
                    Err(err) => println!("Ошибка получения баланса: {}", err),
                }
            },
            
            "list" => {
                let list = storage.get_all();
                for (name, balance, incoming, expense) in list {
                    println!("{}: баланс: {}, приход: {}, расход: {}", name, balance, incoming, expense);
                }
            },
            "exit" => break,
            _ => continue,
        }
    }
    println!("Good Bye!!");
    
} 
