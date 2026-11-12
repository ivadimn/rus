use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::{BufRead, BufReader, Write};

use crate::{Balance, Name};

const LIMIT: i64 = -1000;

#[derive(Debug, Clone)]
pub struct Account {
    pub name: Name,
    pub balance: Balance,
}

impl Account {
    fn new(name: Name) -> Self {
        Account { name, balance: Balance(0) }
    }
}

pub struct Storage {
    accounts: HashMap<Name, Account>,
}

impl Storage {
    pub fn new() -> Self {
        Storage { accounts: HashMap::new() }
    }

    /// Добавляет пользователя. Some(0) — успех, None — уже существует.
    pub fn add_user(&mut self, name: Name) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name.clone(), Account::new(name));
            Some(Balance(0))
        }
    }

    /// Удаляет пользователя. Some(balance) — успех, None — не найден.
    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name).map(|acc| acc.balance)
    }

    /// Возвращает баланс или None, если пользователь не найден.
    pub fn get_balance(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).map(|acc| acc.balance)
    }

    /// Пополняет баланс. Возвращает новый баланс или Err.
    pub fn deposit(&mut self, name: &Name, amount: Balance) -> Result<Balance, String> {
        if amount < Balance(0) {
            return Err("Сумма пополнения не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => { acc.balance.0 += amount.0; Ok(acc.balance) }
            None => Err(format!("Пользователь '{}' не найден", name)),
        }
    }

    /// Снимает средства. Возвращает новый баланс или Err.
    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<Balance, String> {
        if amount < Balance(0) {
            return Err("Сумма снятия не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => {
                if acc.balance - amount < Balance(LIMIT) {
                    Err(format!("Превышен кредитный лимит: лимит {}, запрошено {}",
                        LIMIT, acc.balance - amount))
                } else {
                    acc.balance.0 -= amount.0;
                    Ok(acc.balance)
                }
            }
            None => Err(format!("Пользователь '{}' не найден", name)),
        }
    }

    pub fn get_all(&self) -> Vec<(Name, Balance)> {
        self.accounts.iter()
            .map(|(n, a)| (n.clone(), a.balance))
            .collect()
    } 

    pub fn load_data(filename: &str) -> Self {
        let mut storage = Storage::new();
        if Path::new(filename).exists() {
            let f = fs::File::open(filename).unwrap();
            let reader = BufReader::new(f);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.trim().split(',').collect();
                    if parts.len() == 2 {
                        let name = parts[0].to_string();
                        let balance = Balance(parts[1].parse().unwrap_or(0));
                        storage.add_user(name.clone());
                        let _ = storage.deposit(&name, balance);
                    }
                }
            }
        } else {
            //создаём банк с нуля
            for user in ["Bod", "Alice", "John", "Vasya"] {
                storage.add_user(user.to_string());
            } 
        }
        storage
    }

    pub fn save(&self, filename: &str) {

        let mut buffer = String::new();
        for acc in self.accounts.values() {
            buffer.push_str(&format!("{},{}\n", acc.name, acc.balance.0));
        }
        fs::write(filename, buffer).expect("Не удалось записать файл!");
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

use super::*;

    #[test]
    fn test_new_storage_is_empty() {
        let storage = Storage::new();
        assert_eq!(storage.accounts.len(), 0);
    }

    #[test]
    fn test_add_user_success() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()), Some(Balance(0)));
    }

    #[test]
    fn test_add_user_duplicate() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string());
        assert_eq!(storage.add_user("Alice".to_string()), None);
    }

    #[test]
    fn test_deposit_increases_balance() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        assert_eq!(storage.deposit(&"Bob".to_string(), Balance(500)), Ok(Balance(500)));
    }

    #[test]
    fn test_deposit_negative_fails() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        assert!(storage.deposit(&"Bob".to_string(), Balance(-100)).is_err());
    }

    #[test]
    fn test_withdraw_success() {
        let mut storage = Storage::new();
        storage.add_user("Charlie".to_string());
        storage.deposit(&"Charlie".to_string(), Balance(300)).unwrap();
        assert_eq!(storage.withdraw(&"Charlie".to_string(), Balance(100)), Ok(Balance(200)));
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut storage = Storage::new();
        storage.add_user("Dana".to_string());
        storage.deposit(&"Dana".to_string(), Balance(500)).unwrap();
        // Пытаемся снять больше, чем есть, — ошибка
        assert!(storage.withdraw(&"Dana".to_string(), Balance(2000)).is_err());
        // Баланс не изменился
        assert_eq!(storage.get_balance(&"Dana".to_string()), Some(Balance(500)));
    }

    #[test]
    fn test_withdraw_within_the_limit() {
        let mut storage = Storage::new();
        storage.add_user("Dana".to_string());
        storage.deposit(&"Dana".to_string(), Balance(500)).unwrap();
        // Пытаемся снять больше, чем есть, — ошибка
        assert_eq!(storage.withdraw(&"Dana".to_string(), Balance(1000)), Ok(Balance(-500)));
        // Баланс не изменился
        //assert_eq!(storage.get_balance(&"Dana".to_string()), Some(50));
    }

    #[test]
    fn test_nonexistent_user_operations() {
        let mut storage = Storage::new();
        assert!(storage.deposit(&"Ghost".to_string(), Balance(100)).is_err());
        assert!(storage.withdraw(&"Ghost".to_string(), Balance(50)).is_err());
        assert_eq!(storage.get_balance(&"Ghost".to_string()), None);
        assert_eq!(storage.remove_user(&"Ghost".to_string()), None);
    }

    #[test]
    fn load_data_exiting_file() {
        let file_path = "test_load_1_1_2.csv";
        let mut f = File::create(file_path).unwrap();
        writeln!(f, "John,100").unwrap();
        writeln!(f, "Alice,200").unwrap();
        writeln!(f, "Bob,50").unwrap();

        let storage = Storage::load_data(file_path);
        assert_eq!(storage.get_balance(&"John".to_string()), Some(Balance(100)));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(Balance(200)));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(Balance(50)));

        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn save_into_file() {
        let file_path = "test_save_1_1_2.csv";
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string());
        storage.add_user("Bob".to_string());
        storage.deposit(&"Alice".to_string(), Balance(100)).unwrap();
        storage.deposit(&"Bob".to_string(), Balance(200)).unwrap();
        storage.save(file_path);

        let content = fs::read_to_string(file_path).unwrap();
        let mut lines: Vec<&str> = content.lines().collect();
        lines.sort();
        assert_eq!(lines, vec!["Alice,100", "Bob,200"]);

        fs::remove_file(file_path).unwrap();
        
    }
   
}