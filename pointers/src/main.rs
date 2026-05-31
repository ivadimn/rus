fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} 

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} 

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct Account {
    name: String,
    balance: i64
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance != 0 {
            println!("Предупреждение: баланс не равен 0!!");
        }
        else {
            println!("Счёт безопасно удалён.");
        }
    }
}

fn print_balance(a: &Account) {
    println!("Текущий баланс счёта {}: {}",a.name, a.balance);
}

fn transfer_funds(a_to: &mut Account, a_from: &mut Account, summa: i64) {
    a_to.balance += summa;
    a_from.balance -= summa;
}

fn destroy_account(mut a_del: Account, a_to: &mut Account) {
    a_to.balance += a_del.balance;
    a_del.balance = 0;
}

struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_reate: u32
}

fn account_interest(bank: &mut Bank) {
    for account in bank.accounts.iter_mut() {
        account.balance += account.balance / 100;
    }
}

fn merge_banks(bank_from: Bank, bank_to: &mut Bank) {
    bank_to.credit_rate = bank_from.credit_rate;
    bank_to.debit_reate = bank_from.debit_reate;
    for account in bank_from.accounts {
        bank_to.accounts.push(account)
    }
    
}

fn bank_info(bank: &Bank) {
    println!("Дебит:  {}", bank.debit_reate);
    println!("Кредит: {}", bank.credit_rate);
    for account in bank.accounts.iter() {
        print_balance(account);
    }
}

fn main() { 
    let mut s1 = Account {name: String::from("Счёт 1"), balance: 250};
    let mut s2 = Account {name: String::from("Счёт 2"), balance: 150};
    let mut s3 = Account {name: String::from("Счёт 3"), balance: 0};
    
    transfer_funds(&mut s3, &mut s1, 50);
    destroy_account(s2, &mut s3);
    //print_balance(&s3);
    let mut bank = Bank {accounts: vec![s1, s3], credit_rate: 0, debit_reate: 0};
    let mut bank2 = Bank {accounts: Vec::new(), credit_rate: 0, debit_reate: 0};

    //bank.accounts.push(s1);
    //bank.accounts.push(s2);
    //bank.accounts.push(s3);
    bank_info(&bank);
    merge_banks(bank, &mut bank2);
    bank_info(&bank2);


}
