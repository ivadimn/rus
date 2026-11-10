use std::collections::HashMap;

pub type Name = String;
pub type Balance = i64;

#[derive(Debug, Clone)]
pub struct Account {
    pub name: Name,
    pub balance: Balance,
}

impl Account {
    fn new(name: Name) -> Self {
        Account { name, balance: 0 }
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
            Some(0)
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
        if amount < 0 {
            return Err("Сумма пополнения не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => { acc.balance += amount; Ok(acc.balance) }
            None => Err(format!("Пользователь '{}' не найден", name)),
        }
    }

    /// Снимает средства. Возвращает новый баланс или Err.
    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<Balance, String> {
        if amount < 0 {
            return Err("Сумма снятия не может быть отрицательной".into());
        }
        match self.accounts.get_mut(name) {
            Some(acc) => {
                if acc.balance < amount {
                    Err(format!("Недостаточно средств: баланс {}, запрошено {}",
                        acc.balance, amount))
                } else {
                    acc.balance -= amount;
                    Ok(acc.balance)
                }
            }
            None => Err(format!("Пользователь '{}' не найден", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_storage_is_empty() {
        let storage = Storage::new();
        assert_eq!(storage.accounts.len(), 0);
    }

    #[test]
    fn test_add_user_success() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()), Some(0));
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
        assert_eq!(storage.deposit(&"Bob".to_string(), 500), Ok(500));
    }

    #[test]
    fn test_deposit_negative_fails() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        assert!(storage.deposit(&"Bob".to_string(), -100).is_err());
    }

    #[test]
    fn test_withdraw_success() {
        let mut storage = Storage::new();
        storage.add_user("Charlie".to_string());
        storage.deposit(&"Charlie".to_string(), 300).unwrap();
        assert_eq!(storage.withdraw(&"Charlie".to_string(), 100), Ok(200));
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut storage = Storage::new();
        storage.add_user("Dana".to_string());
        storage.deposit(&"Dana".to_string(), 50).unwrap();
        // Пытаемся снять больше, чем есть, — ошибка
        assert!(storage.withdraw(&"Dana".to_string(), 100).is_err());
        // Баланс не изменился
        assert_eq!(storage.get_balance(&"Dana".to_string()), Some(50));
    }

    #[test]
    fn test_nonexistent_user_operations() {
        let mut storage = Storage::new();
        assert!(storage.deposit(&"Ghost".to_string(), 100).is_err());
        assert!(storage.withdraw(&"Ghost".to_string(), 50).is_err());
        assert_eq!(storage.get_balance(&"Ghost".to_string()), None);
        assert_eq!(storage.remove_user(&"Ghost".to_string()), None);
    }
}
