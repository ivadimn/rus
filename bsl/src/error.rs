use std::fmt::Display;
use std::error::Error;

use crate::storage::Name;
use crate::balance::Balance;

#[derive(Debug)]
pub enum StorageError {
    UserNotFound(Name),
    UserExists(Name),
    NotEnoughMoney{required: Balance, avalaible: Balance},
    NegativeAmount(Balance)
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserNotFound(user) => write!(f, "Пользователь: {} не найден.", user),
            Self::NotEnoughMoney { required, avalaible } => write!(f, "Превышен кредитный лимит: лимит {}, запрошено {}",
                avalaible, required),
            Self::UserExists(user) => write!(f, "Пользователь: {} уже существует.", user),
            Self::NegativeAmount(amount) => write!(f, "Сумма пополнения: {} не может быть отрицательной", amount),
        }
    }
}

impl Error for StorageError {}