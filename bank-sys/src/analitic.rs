
use crate::{Name, Balance, Storage};

pub fn max_profit(storage: &Storage) -> Option<(Name, Balance)> {

    let accounts = storage.get_accounts();
    accounts.iter()
        .map(|(n, a)| (n.clone(), a.incoming - a.expense))
        .max_by(|a1, a2| a1.1.0.partial_cmp(&a2.1.0).unwrap())
}