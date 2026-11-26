use std::ops::Add;

use bsl_macros::{ Describable, Transaction};

use crate::storage::{Name, Storage};
use crate::balance::Balance;
use crate::error::StorageError;
use crate::{impl_add, transaction};

pub trait Transaction {
    fn apply(&self, storage: &mut Storage) -> Result<(), StorageError>;
}

pub trait Describable {
    fn describe(&self) -> String;
}

#[derive(Describable, Transaction)]
pub struct Deposit {
    pub account: Name,
    pub amount: Balance,
}

#[derive(Transaction)]
#[transaction("transfer")]
pub struct Transfer { 
    pub from: Name,
    pub to: Name,
    pub amount: Balance,
}

#[derive(Transaction)]
#[transaction("withdraw")]
pub struct Withdraw {
    pub account: Name,
    pub amount: Balance,
}


pub struct TxCombinator<T: Transaction, U: Transaction> {
    pub tx1: T,
    pub tx2: U,
}

impl<T: Transaction, U: Transaction> Transaction for TxCombinator<T, U>  {
    fn apply(&self, storage: &mut Storage) -> Result<(), StorageError> {
        self.tx1.apply(storage)?;
        self.tx2.apply(storage)?;
        Ok(())
    }
}

// impl Transaction for Deposit {
//     fn apply(&self, storage: &mut Storage) -> Result<(), StorageError> {
//         match  storage.get_accounts().get(&self.account) {

//             Some(_) => storage.deposit(&self.account, self.amount)?, 
//             None => return Err(StorageError::UserNotFound(self.account.clone())),
//         };
//         Ok(())
//     }
// }

// impl Transaction for Transfer {
//     fn apply(&self, storage: &mut Storage) -> Result<(), StorageError> {
        
//         if !storage.get_accounts().contains_key(&self.to) {
//             return Err(StorageError::UserNotFound(self.to.clone()));
//         }

//         storage.withdraw(&self.from, self.amount)?;
//         storage.deposit(&self.to, self.amount)?;
        
//         Ok(())
//     }
// }

// impl Transaction for Withdraw  {
//     fn apply(&self, storage: &mut Storage) -> Result<(), StorageError> {
//         match  storage.get_accounts().get(&self.account) {

//             Some(_) => storage.withdraw(&self.account, self.amount)?, 
//             None => return Err(StorageError::UserNotFound(self.account.clone())),
//         };
//         Ok(())
//     }
// }
 
/* impl Add<Deposit> for Transfer {
    type Output = TxCombinator<Transfer, Deposit>;

    fn add(self, rhs: Deposit) -> Self::Output {
        TxCombinator {tx1: self, tx2: rhs}
    }
}

impl Add<Transfer> for Deposit {
    type Output = TxCombinator<Deposit, Transfer>;

    fn add(self, rhs: Transfer) -> Self::Output {
        TxCombinator {tx1: self, tx2: rhs}
    }
}

impl Add<Deposit> for Deposit {
    type Output = TxCombinator<Deposit, Deposit>;
    fn add(self, rhs: Deposit) -> Self::Output {
        TxCombinator {tx1: self, tx2: rhs}
    }    


}

impl Add<Transfer> for Transfer {
    type Output = TxCombinator<Transfer, Transfer>;

    fn add(self, rhs: Transfer) -> Self::Output {
        TxCombinator {tx1: self, tx2: rhs}
    }
} */

impl_add! {
    (Deposit, Transfer),
    (Transfer, Deposit),
    (Deposit, Deposit),
    (Transfer, Transfer),
}

