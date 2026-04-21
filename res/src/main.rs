

struct Account {
    balance: i64,
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance != 0 {
            println!("Предупреждение: Баланс не равен нулю! {}", self.balance);
        } else {
            println!("Счёт успешно удалён!");
        }
    }
}

fn print_balance(a: Account) {
    println!("Текущий баланнс счёта: {}", a.balance);
}

fn transfer_balance(a1: &mut Account, a2: &mut Account, summa: i64) {
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

    let s2 = String::from("hello");

    let r1 = &s2;
    let r2 = &s2;

    //let s3 = takes_and_gives_back(s2);

    println!("{r1} {r2}");
    println!("{r1} {r2}");
}
