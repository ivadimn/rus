

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

fn main() {
    let a1 = Account {balance: 200};
    let mut a2 = Account {balance: 0};
    //let a3 = Account {balance: 40000};
    //let a1 = Account {balance: 0};
    destroy_account(a1,  &mut a2);
    print_balance(a2);
}
