

struct Account {
    balance: f64,
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance != 0.0 {
            println!("Предупреждение: Баланс не равен нулю! {}", self.balance);
        } else {
            println!("Счёт успешно удалён!");
        }
    }
}


struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_rate: u32 
}


fn accrue_interest(bank: &mut Bank) {
    for account in bank.accounts.iter_mut() {
        account.balance += account.balance * 0.01;
    }
}

fn print_bank_balance(bank: &Bank) {
    for account in bank.accounts.iter() {
        print_balance(account);
    }
}

fn print_balance(a: &Account) {
    println!("Текущий баланнс счёта: {}", a.balance);
}

fn transfer_balance(a1: &mut Account, a2: &mut Account, summa: f64) {
    a1.balance -= summa;
    a2.balance += summa;

}

fn destroy_account(mut d: Account, a: &mut Account) {
    let summa = d.balance;
    transfer_balance(&mut d, a, summa);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn main() {
    //let s1 = gives_ownership();

    // let mut s = String::from("hello");

    // s.push_str(", world!");
    // println!("{s}");
    // let s1 = s.clone();
    // println!("{s1}");
    // println!("{s}");

    let mut a1 = Account{balance: 250.0};
    let a2 = Account{balance: 100.0};
    let mut a3 = Account{balance: 280.0};

    print_balance(&a2);
    //transfer_balance(&mut a1, &mut a3, 250.0);
    

    let mut bank = Bank {accounts: Vec::new(), credit_rate: 0, debit_rate: 0};
    bank.accounts.push(a1);
    bank.accounts.push(a2);
    bank.accounts.push(a3);

    accrue_interest(&mut bank);
    print_bank_balance(&bank);

    
}
