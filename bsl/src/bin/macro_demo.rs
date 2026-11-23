// bin/macros_demo.rs

use bsl::{tx_chain, create_users};
use bsl::Storage;
use bsl::transaction::{Deposit, Transaction, Transfer, Withdraw};
use bsl::balance::Balance;

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
}