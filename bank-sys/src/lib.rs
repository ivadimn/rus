pub mod storage;
pub mod analitic; 

pub use storage::Storage;

pub type Name = String;
use std::ops::{Sub, Add, AddAssign, SubAssign};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Operation {
    Deposit(u32),
    Withdraw(u32),
    CloseAccount
}

// struct Transaction {
//     operation: Operation,
// }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Balance(pub i64);

impl Balance {
    pub fn try_apply(&self, ops: Vec<Operation>) -> Vec<Operation> {
        let mut err_ops: Vec<Operation> = Vec::new();

        for op in &ops {
            match &op {
                Operation::Deposit(amount) => {
                    if *amount == 0 {
                        err_ops.push(op.clone());
                    }
                },
                Operation::Withdraw(amount) => {
                    if *amount as i64 > self.0 {
                        err_ops.push(op.clone());
                    }
                },
                Operation::CloseAccount => { err_ops.push(op.clone()); }
            }
        }
        err_ops
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub for Balance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Add for Balance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
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
