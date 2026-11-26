// bin/macros_demo.rs

use bsl::{tx_chain, create_users};
use bsl::transaction::{Describable, Transaction};
use bsl::Storage;
use bsl::transaction::{Deposit, Transfer, Withdraw};
use bsl::balance::Balance;
use bsl_macros::{say_hello, Describable};

fn main() {
    let mut storage = Storage::new();
    create_users!(storage, "Alice", "Bob");
    //storage.add_user("Alice".into()).unwrap();
    //storage.add_user("Bob".into()).unwrap();

    let tx = tx_chain!(
        Deposit { account: "Alice".into(), amount: Balance(500) },
        Transfer { from: "Alice".into(), to: "Bob".into(), amount: Balance(50) },
        Withdraw { account: "Alice".into(), amount: Balance(100) }
    );

    match tx.apply(&mut storage) {
        Ok(_) => println!("Успешно"),
        Err(e) => println!("Ошибка: {:?}", e),
    }

    for (name, balance, inc, exp) in storage.get_all() {
        println!("{} -> {}, {}, {}", name, balance, inc, exp);
    }

    say_hello!("Привет из процедурного макроса");

    let d = Deposit { account: "Vasia".into(), amount: Balance(100) };
    storage.add_user("Vasia".into()).unwrap();
       println!("{}", d.describe()); // → Я — структура Deposit
    let _ = d.apply(&mut storage);
    

}