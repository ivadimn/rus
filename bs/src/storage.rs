
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;


use crate::balance::Balance;
use crate::error::StorageError;

pub type Name = String;

const LIMIT: i64 = -1000;

#[derive(Debug, Clone)]
pub enum Operation {
    Deposit(u32),
    Withdraw(u32),
    CloseAccount
}

#[derive(Debug, Clone)]
pub struct Account {
    pub name: Name,
    pub balance: Balance,
    pub incoming : Balance,
    pub expense: Balance,
}

impl Account {
    fn new(name: Name) -> Self {
        Account { name, 
            balance: Balance(0),
            incoming: Balance(0),
            expense: Balance(0),
        }
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
    pub fn add_user(&mut self, name: Name) -> Result<Balance, StorageError> {
        if self.accounts.contains_key(&name) {
            Err(StorageError::UserExists(name.into()))
        } else {
            self.accounts.insert(name.clone(), Account::new(name));
            Ok(Balance(0))
        }
    }

    /// Удаляет пользователя. Some(balance) — успех, None — не найден.
    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name).map(|acc| acc.balance)
    }

    /// Возвращает баланс или None, если пользователь не найден.
    pub fn get_balance(&self, name: &Name) -> Result<Balance, StorageError> {
        self.accounts.get(name)
            .map(|acc| acc.balance)
            .ok_or(StorageError::UserNotFound(name.into()))
        
    }

    ///Возвращает общий доход или None, если пользователь не найден
    pub fn get_incoming(&self, name: &Name) -> Result<Balance, StorageError> {
        self.accounts.get(name)
            .map(|acc| acc.incoming)
            .ok_or(StorageError::UserNotFound(name.into()))
    }

    ///Возвращает общий расход или None, если пользователь не найден
    pub fn get_expense(&self, name: &Name) -> Result<Balance, StorageError> {
        self.accounts.get(name)
            .map(|acc| acc.expense)
            .ok_or(StorageError::UserNotFound(name.into()))
    }
    
    //восстанавливает данные пользователя из файла
    pub fn restore_user(&mut self, name: &Name, 
            balance: Balance, incoming: Balance, expense: Balance) -> Result<Balance, StorageError> {
        match self.accounts.get_mut(name) {
            Some(acc) => {
                acc.balance = balance;
                acc.incoming = incoming;
                acc.expense = expense;
                Ok(acc.balance)
            },
            None => Err(StorageError::UserNotFound(name.into()))
        }  
    }

    /// Пополняет баланс. Возвращает новый баланс или Err.
    pub fn deposit(&mut self, name: &Name, amount: Balance) -> Result<Balance, StorageError> {
        if amount < Balance(0) {
            return Err(StorageError::NegativeAmount(amount));
        }
        match self.accounts.get_mut(name) {
            Some(acc) => { 
                acc.balance += amount; 
                acc.incoming += amount;
                Ok(acc.balance) }
            None => Err(StorageError::UserNotFound(name.into())),
        }
    }

    /// Снимает средства. Возвращает новый баланс или Err.
    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<Balance, StorageError> {
        if amount < Balance(0) {
            return Err(StorageError::NegativeAmount(amount));
        }
        match self.accounts.get_mut(name) {
            Some(acc) => {
                if acc.balance - amount < Balance(LIMIT) {
                    Err(StorageError::NotEnoughMoney { required: amount, avalaible: Balance(LIMIT)})
                } else {
                    acc.balance -= amount;
                    acc.expense += amount;
                    Ok(acc.balance)
                }
            }
            None => Err(StorageError::UserNotFound(name.into())),
        }
    }

    pub fn get_all(&self) -> Vec<(Name, Balance, Balance, Balance)> {
        self.accounts.iter()
            .map(|(n, a)| (n.clone(), a.balance, a.incoming, a.expense))
            .collect()
    } 
    pub fn get_accounts(&self) -> &HashMap<Name, Account> {
        &self.accounts
    }

    pub fn load_data(filename: &str) -> Self {
        let mut storage = Storage::new();
        if Path::new(filename).exists() {
            let f = fs::File::open(filename).unwrap();
            let reader = BufReader::new(f);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.trim().split(',').collect();
                    if parts.len() == 4 {
                        let name = parts[0].to_string();
                        let balance = Balance(parts[1].parse().unwrap_or(0));
                        let incoming = Balance(parts[2].parse().unwrap_or(0));
                        let expense = Balance(parts[3].parse().unwrap_or(0));
                        if let Err(err) = storage.add_user(name.clone()) {
                            println!("{}", err);
                        } else {
                            let _ = storage.restore_user(&name, balance, incoming, expense);
                        }
                    }
                }
            }
        }
        // else {
        //     //создаём банк с нуля
        //     for user in ["Bod", "Alice", "John", "Vasya"] {
        //         storage.add_user(user.to_string()).unwrap();
        //     } 
        // }
        storage
    }

    pub fn save(&self, filename: &str) {

        let mut buffer = String::new();
        for acc in self.accounts.values() {
            buffer.push_str(&format!("{},{},{},{}\n", 
                acc.name, acc.balance.0, acc.incoming.0, acc.expense.0));
        }
        fs::write(filename, buffer).expect("Не удалось записать файл!");
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{BufWriter, Cursor}};

use super::*;

    #[test]
    fn test_new_storage_is_empty() {
        let storage = Storage::new();
        assert_eq!(storage.accounts.len(), 0);
    }

    #[test]
    fn test_add_user_success() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()).unwrap(), Balance(0));
    }

    #[test]
    fn test_add_user_duplicate() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string());
        assert!(storage.add_user("Alice".to_string()).is_err());
    }

    #[test]
    fn test_deposit_increases_balance() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        assert_eq!(storage.deposit(&"Bob".to_string(), Balance(500)).unwrap(), Balance(500));
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
        assert_eq!(storage.withdraw(&"Charlie".to_string(), Balance(100)).unwrap(), Balance(200));
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut storage = Storage::new();
        storage.add_user("Dana".to_string());
        storage.deposit(&"Dana".to_string(), Balance(500)).unwrap();
        // Пытаемся снять больше, чем есть, — ошибка
        assert!(storage.withdraw(&"Dana".to_string(), Balance(2000)).is_err());
        // Баланс не изменился
        assert_eq!(storage.get_balance(&"Dana".to_string()).unwrap(), Balance(500));
    }

    #[test]
    fn test_withdraw_within_the_limit() {
        let mut storage = Storage::new();
        storage.add_user("Dana".to_string());
        storage.deposit(&"Dana".to_string(), Balance(500)).unwrap();
        // Пытаемся снять больше, чем есть, — ошибка
        assert_eq!(storage.withdraw(&"Dana".to_string(), Balance(1000)).unwrap(),Balance(-500));
        // Баланс не изменился
        //assert_eq!(storage.get_balance(&"Dana".to_string()), Some(50));
    }

    #[test]
    fn test_nonexistent_user_operations() {
        let mut storage = Storage::new();
        assert!(storage.deposit(&"Ghost".to_string(), Balance(100)).is_err());
        assert!(storage.withdraw(&"Ghost".to_string(), Balance(50)).is_err());
        assert!(storage.get_balance(&"Ghost".to_string()).is_err());
        assert_eq!(storage.remove_user(&"Ghost".to_string()), None);
    }

    #[test]
    fn test_load_data_exiting_file() {
        let file_path = "test_load_1_1_2.csv";
        let mut f = File::create(file_path).unwrap();
        writeln!(f, "John,100,200,100").unwrap();
        writeln!(f, "Alice,200,300,100").unwrap();
        writeln!(f, "Bob,50,100,50").unwrap();

        let storage = Storage::load_data(file_path);
        assert_eq!(storage.get_balance(&"John".to_string()).unwrap(), Balance(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()).unwrap(),Balance(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()).unwrap(), Balance(50));

        assert!(storage.get_balance(&"Vasya".to_string()).is_err());

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_data_from_cursor() {
        let data = b"John,100,200,100\nAlice,200,300,100\nBob,50,100,50";
        let cursor = Cursor::new(data);

        let mut storage = Storage::new();

        let reader = BufReader::new(cursor);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.trim().split(',').collect();
                if parts.len() == 4 {
                    let name = parts[0].to_string();
                    let balance = Balance(parts[1].parse().unwrap_or(0));
                    let incoming = Balance(parts[2].parse().unwrap_or(0));
                    let expense = Balance(parts[3].parse().unwrap_or(0));
                    storage.add_user(name.clone());
                    storage.restore_user(&name, balance, incoming, expense).unwrap();
                }
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()).unwrap(), Balance(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()).unwrap(), Balance(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()).unwrap(), Balance(50));
    }

    #[test]
    fn test_save_into_file() {
        let file_path = "test_save_1_1_2.csv";
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string()).unwrap();
        storage.add_user("Bob".to_string()).unwrap();
        storage.deposit(&"Alice".to_string(), Balance(100)).unwrap();
        storage.deposit(&"Bob".to_string(), Balance(200)).unwrap();
        storage.save(file_path);

        let content = fs::read_to_string(file_path).unwrap();
        let mut lines: Vec<&str> = content.lines().collect();
        lines.sort();
        assert_eq!(lines, vec!["Alice,100,100,0", "Bob,200,200,0"]);

        fs::remove_file(file_path).unwrap();
        
    }

    #[test]
    fn test_save_into_cursor() {
        let mut storage = Storage::new();
        storage.add_user("Alice".to_string()).unwrap();
        storage.add_user("Bob".to_string()).unwrap();
        storage.deposit(&"Alice".to_string(), Balance(100)).unwrap();
        storage.deposit(&"Bob".to_string(), Balance(200)).unwrap();

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);
        {
            let mut writer = BufWriter::new(&mut cursor);
            for (name, balance, incoming, expense) in storage.get_all() {
                writeln!(writer, "{},{},{},{}", name, balance.0, incoming.0, expense.0).unwrap();
            }
            writer.flush().unwrap();
        }
        
        cursor.set_position(0);
        let mut lines: Vec<String> = BufReader::new(cursor)
            .lines()
            .map(|l| l.unwrap())
            .collect();
        lines.sort();

        assert_eq!(lines, vec!["Alice,100,100,0", "Bob,200,200,0"]);
    }
}
