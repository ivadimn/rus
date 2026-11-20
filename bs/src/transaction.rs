#[derive(Debug)]
enum TxError {
    InsufficientFunds,
    InvalidAccount,
}
use crate::storage::Storage;

trait Trensaction {
    fn apply(&self, accounts: &Storage);
}