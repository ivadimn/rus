
mod random_info;
use random_info::*;

#[derive(Clone)]
pub struct Foo {
    x: u32,
    y: u32,
    z: Bar,
} 

use std::fmt;

use crate::Payment::Cash;

impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Foo")
        .field("x", &self.x)
        .field("y", &self.y)
        .field("z", &self.z.x)
        .finish()
    }
}

#[derive(Clone)]
struct Bar {
    x: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Data {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    random: RandomInfo,
}

impl RandomInfo {
    pub fn is_large(&mut self, compare_to: i64) -> bool {
        self.call_count += 1;
        self.some_int > compare_to
    }
}

impl SomeTrait for Data {
    fn is_valid(&self) -> bool {
        true
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 10.3,
            some_int: 80,
            random: RandomInfo::new(true),
        }
    }
}


fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yes");
    }
}

enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto{account_id: String, amount: f32},
}

struct DebitData {
    pub card_number: String,
    pub amount: f32,
}

#[allow(unused_variables)]
fn main() {
    let example_str = "Howdy";
    let example_string = String::from("Partner");

    let string_from_str = example_str.to_string();
    let string_from_str2 = "Some hardcoded string".to_string();    

    let string_from_hardcoded = String::from("Some hardcoded string");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_str;

    let combine_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "first", "second");

    let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcode literal");
    mut_string.push('m');

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;

    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(ch) => println!("Found a char {}", ch),
        None => {}
    }

    if let Some(ch) = example_str.chars().nth(2) {
        println!("Found a char {}", ch);
    }

    let string_slice_var = "Howdy";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a real string");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);

    some_string_procedure(string_var);
    //some_string_procedure(string_var);

    let some_tuple = (2, 3.4);
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("My data is {:?}", some_tuple);

    let data_var = Data::default();


    let mut random_info_var = RandomInfo {
        call_count: 0,
        some_bool: true,
        some_int: 890,
    };

    let is_this_smaller = random_info_var.is_smaller(9);
    let is_this_large = random_info_var.is_large(70);
    let is_valid = random_info_var.is_valid();

    print_if_is_valid(&random_info_var);
    print_if_is_valid(&data_var);

    let some_payment = Payment::Cash(100.);
    let debit_payment = Payment::DebitCard(DebitData {card_number: String::from("123456789"), amount: 233.});

    process_payment(some_payment);
    process_payment(debit_payment);
    
}


fn process_payment(some_payment: Payment) {
    match some_payment {
        Payment::Cash(s) => println!("Paying with cash...{}", s),
        Payment::CreditCard(some_string, some_f32) => 
            println!("Paying with credit card {} {}", some_string, some_f32),
        Payment::DebitCard(debit) => 
            println!("Paying with debit card {} {}", debit.card_number, debit.amount),
        Payment::Crypto { account_id, amount } => 
            println!("Crypto {} {}", account_id, amount),
    }
}

fn some_str_procedure(param: &str) {
    println!("I'm some_str_procedure {}", param);
}

fn some_string_procedure(param: String) {
    println!("I'm some_str_procedure {}", param);
}

fn get_some_rgb() -> (u8, u8, u8) {
    (200, 100, 20)
}