
pub enum Operation {
    Incom(i64),
    Expense(i64),
}

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

pub struct Sum(pub i64);

pub struct Transaction {
    date: Date,
    category: String,
    Operation: Operation,
    amount: Sum,
}

