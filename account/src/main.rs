use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
pub enum OpKind {
    // пополнить/потратить счёт
    Deposit(u32),
    Withdraw(u32),
    // закрыть аккаунт - все средства выведены
    CloseAccount,
} // вот и всё, никаких посторонних операций и данных!

mod bank_system {
    use std::collections::HashMap;
    use std::fs;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use super::OpKind;

    pub type Name = String;
    
    #[derive(Debug, Clone)]
    pub struct Balance {
        pub result: i64,
        pub last_ops: Vec<OpKind>,
    }
    impl Balance {
        pub fn new(result: i64) -> Self {
            Self{result, last_ops: vec![]}
        }
        // Можно пойти ещё дальше и в качестве аргумента принимать любой тип,
        // который может итерироваться по OpKind, с помощью дженерика:
        // fn process<'a>(&mut self, impl IntoIterator<Item=&'a OpKind>) -> Vec<&'a OpKind>
        // Пробуйте, дерзайте!
        pub fn process<'a>(&mut self, ops: &[&'a OpKind]) -> Vec<&'a OpKind> {
            let mut remaining = ops.into_iter();
            let mut bad_ops = Vec::new();
            for op in &mut remaining {
                match op {
                    OpKind::Deposit(value) => {
                        self.result += *value as i64;
                    },
                    OpKind::Withdraw(value) if self.result > *value as i64 => {
                        self.result -= *value as i64;
                    },
                    other @ _ => {
                        bad_ops.push(*other);
                        break;
                    },
                }
            }
            bad_ops.extend(remaining);
            bad_ops
        }
    }

    #[derive(Debug, Clone)]
    pub struct Account {
        pub name: Name,
        pub balance: Balance,
    }
    
    impl Account {
        fn new(name: Name) -> Self {
            Account { name, balance: Balance::new(0) }
        }
    }
    
    pub struct Storage {
        pub accounts: HashMap<Name, Account>,
    }
    
    impl Storage {
        pub fn new() -> Self {
            Storage { accounts: HashMap::new() }
        }
        pub fn accounts(&self) -> &HashMap<Name, Account> {
            &self.accounts
        }
    
        /// Добавляет пользователя. Some(0) — успех, None — уже существует.
        pub fn add_user(&mut self, name: Name) -> Option<Balance> {
            if self.accounts.contains_key(&name) {
                None
            } else {
                self.accounts.insert(name.clone(), Account::new(name));
                Some(Balance::new(0))
            }
        }
    
        /// Удаляет пользователя. Some(balance) — успех, None — не найден.
        pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
            self.accounts.remove(name).map(|acc| acc.balance)
        }
    
        /// Возвращает баланс или None если пользователь не найден.
        pub fn get_balance(&self, name: &Name) -> Option<&Balance> {
            self.accounts.get(name).map(|acc| &acc.balance)
        }
    
        /// Пополняет баланс. Возвращает новый баланс или Err.
        pub fn deposit(&mut self, name: &Name, amount: i64) -> Result<&Balance, String> {
            if amount < 0 {
                return Err("Сумма пополнения не может быть отрицательной".into());
            }
            match self.accounts.get_mut(name) {
                Some(acc) => { acc.balance.result += amount; Ok(&acc.balance) }
                None => Err(format!("Пользователь '{}' не найден", name)),
            }
        }
    
        /// Снимает средства. Возвращает новый баланс или Err.
        pub fn withdraw(&mut self, name: &Name, amount: i64) -> Result<&Balance, String> {
            if amount < 0 {
                return Err("Сумма снятия не может быть отрицательной".into());
            }
            match self.accounts.get_mut(name) {
                Some(acc) => {
                    if acc.balance.result < amount {
                        Err(format!("Недостаточно средств: баланс {}, запрошено {}",
                            acc.balance.result, amount))
                    } else {
                        acc.balance.result -= amount;
                        Ok(&acc.balance)
                    }
                }
                None => Err(format!("Пользователь '{}' не найден", name)),
            }
        }
        
        /// Возвращает всех пользователей и их балансы.
        pub fn get_all(&self) -> Vec<(Name, Balance)> {
            self.accounts
                .iter()
                .map(|(n, acc)| (n.clone(), acc.balance.clone()))
                .collect()
        }
    
        /// Загружает данные из CSV-файла формата "Name,Balance".
        /// Если файл не существует — создаёт хранилище с пользователями по умолчанию.
        pub fn load_data(file: &str) -> Storage {
            let mut storage = Storage::new();
    
            if Path::new(file).exists() {
                let f = fs::File::open(file).unwrap();
                // BufReader читает данные блоками, а не байт за байтом —
                // это делает построчное чтение через .lines() быстрым и эффективным
                let reader = BufReader::new(f);
    
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let parts: Vec<&str> = line.trim().split(',').collect();
                        if parts.len() == 2 {
                            let name = parts[0].to_string();
                            let balance: i64 = parts[1].parse().unwrap_or(0);
                            storage.add_user(name.clone());
                            let _ = storage.deposit(&name, balance);
                        }
                    }
                }
            } else {
                // Файла нет — создаём пользователей с нуля
                for u in ["John", "Alice", "Bob", "Vasya"] {
                    storage.add_user(u.to_string());
                }
            }
    
            storage
        }
    
        /// Сохраняет текущее состояние в CSV-файл формата "Name,Balance".
        /// fs::write записывает все данные одним вызовом — эффективно для небольших файлов.
        pub fn save(&self, file: &str) {
            let mut data = String::new();
            for (name, balance) in self.get_all() {
                data.push_str(&format!("{},{}\n", name, balance.result));
            }
            fs::write(file, data).expect("Не удалось записать файл");
        }
    }
}
pub use bank_system::{Name, Storage, Balance, Account};

mod analysis {
    use super::{Storage, OpKind, Balance};

    pub fn find_best<'a>(storage: &'a Storage) -> Option<(&'a str, f32)> {
        if storage.accounts().is_empty() {
            return None;
        }
        let mut best_factor = f32::MIN;
        let mut best_name = "";
        for (name, account) in storage.accounts() {
            let mut all_positive = 0;
            for op in &account.balance.last_ops {
                match op {
                    OpKind::Deposit(value) => all_positive += *value as u64,
                    _ => (),
                }
            }
            // почти то же самое на итераторах!
            let all_negative: u64
                = account.balance.last_ops.iter()
                  .filter_map(
                      |op| match op {OpKind::Withdraw(value) => Some(*value as u64), _ => None}
                      )
                  .sum();
            let factor = all_positive as f32 / all_negative as f32;
            if factor > best_factor {
                best_factor = factor;
                best_name = &name;
            }
        }
        Some((best_name, best_factor))
    }
}
/*
fn main () {
    let ops = [&OpKind::Deposit(32), &OpKind::Withdraw(64), &OpKind::CloseAccount];
    let bad_ops = Balance::new(0).process(&ops);
    assert_eq!(bad_ops.len(), 2);
    println!("{:?}", bad_ops);
}*/

fn make_init(name: String, balance: Balance) -> (String, Account) {
    (name.clone(), Account{name, balance})
}

fn main () {
    // Для упрощения. Добавлять можно и по-одному
    let storage = Storage {accounts: [
        make_init( "Dad".to_string(),
                   Balance{ result: 0,
                            last_ops: vec![ OpKind::Deposit(200000),
                                            OpKind::Withdraw(100000) ] } ),
        make_init( "Mom".to_string(),
                   Balance{ result: 0,
                            last_ops: vec![ OpKind::Deposit(120000),
                                            OpKind::Withdraw(50000),
                                            OpKind::Withdraw(20000) ] } ),
        make_init( "Son".to_string(),
                   Balance{ result: 0,
                            last_ops: vec![ OpKind::Deposit(5000),
                                            OpKind::Withdraw(500),
                                            OpKind::Withdraw(1000),
                                            OpKind::Withdraw(700) ] } ),
    ].into_iter().collect()};
    println!(r#"best factor for "{}"!"#, analysis::find_best(&storage).unwrap().0);
}