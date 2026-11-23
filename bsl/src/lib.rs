pub mod storage;
pub mod balance;
pub mod error;
pub mod transaction;

pub use storage::Storage;
pub use transaction::TxCombinator;
//pub use balance::Balance;

#[macro_export]
macro_rules! impl_add {
    ( $( ($lhs:ty, $rhs:ty) ),* $(,)?) => {
        $ (
            impl Add<$rhs> for $lhs {
                type Output = $crate::TxCombinator<$lhs, $rhs>;
                fn add(self, rhs: $rhs) -> Self::Output {
                    $crate::TxCombinator {tx1: self, tx2: rhs }
                }
            }
         )*
    };
}

#[macro_export]
macro_rules! tx_chain {
    ( $first:expr $(, $rest:expr )* $(,)? ) => {{
        let tx = $first;
        $(
            let tx = $crate::TxCombinator { tx1: tx, tx2: $rest };
        )*
        tx
    }};
} 

#[macro_export]
macro_rules! create_users {
    ($storage:expr, $( $name:expr ),+ $(,)?) => {
        $ (
            $storage.add_user($name.into()).unwrap();
        )*
    };
}
