#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_imports)]
use std::io::{self, Write};

use bsl::{Storage};
use bsl::balance::Balance;
use bsl::transaction::{Deposit, Transaction, Transfer, Withdraw, TxCombinator};
use bsl::tx_chain;

fn help_print() {
    println!("\nВведите команду:");
    println!("add <username> <summa>       - добавить пользователя.");
    println!("deposit <username> <summa>   - положить деньги.");
    println!("withdraw <username> <summa>  - снять деньги.");
    println!("transfer <from> <to> <summa> - перевести деньги со счёта на счёт.");
    println!("balance <username>           - показать баланс пользователя.");
    println!("list                         - показать список пользователей.");
    println!("exit                         - закрыть программу.");
    //println!("add <username> - добавить пользователя.");
    
}

fn main() {

    let filename = "bank.csv";
    println!("Программа обслуживания банковских счетов.");
    let mut storage = Storage::load_data(filename);

    let tx = tx_chain!(
        Deposit { account: "Alice".into(), amount: Balance(500) },
        Transfer { from: "Alice".into(), to: "Bob".into(), amount: Balance(50) },
        Withdraw { account: "Alice".into(), amount: Balance(100) }
    );
    
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

                let dep = Deposit {
                    account: name.clone(),
                    amount: Balance(balance),
                };
                
                match dep.apply(&mut storage) {
                    Ok(_) => {
                        println!("Баланс пользователе с именем {} увеличен текущзий баланс: {}", 
                            name, storage.get_balance(&name).unwrap());
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

                let wd = Withdraw {
                    account:  name.clone(),
                    amount: Balance(balance),
                };

                match wd.apply(&mut storage) {
                    Ok(_) => {
                        println!("Деньги в сумме: {} списаны со счёта: {}", 
                            balance, name );
                        storage.save(filename);

                    },
                    Err(err) => println!("Ошибка списания средств: {}", err),
                }
            },
            "transfer" => {
                if args.len() != 4 {
                    println!("Пример использования: transfer Alice Bob 100");
                    continue;
                }
                let from = args[1].to_string();
                let to = args[2].to_string();
                let Ok(balance) = args[3].parse::<i64>() else {
                    println!("Не правильно введена сумма!");
                    continue;
                };

                let tx = Transfer {
                    from: from.clone(),
                    to: to.clone(),
                    amount: Balance(balance),
                };
                
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Деньги в сумме {} перведены со счёта: {} на счёт: {}", 
                            balance, from, to);
                        storage.save(filename);

                    },
                    Err(err) => println!("Ошибка перевода средств: {}", err),
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