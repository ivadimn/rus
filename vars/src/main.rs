
use std::io;

#[derive(Debug)]
pub struct Moo {
    a: u32,
    b: u64,
}

#[derive(Debug)]
pub enum Foo {
    Bar,
    Baz(u32, u64),
    Zoo {
        val: u32,
        flag:bool,
    },
    Moo(Moo),
}



fn main() {
    
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("input failure");
    let number :u32 = number.trim().parse().expect("bad number");
    let mut summa: u32 = 0;
    summa += number % 10;
    let number = number / 10;
    summa += number % 10;
    let number = number / 10;
    summa += number % 10;
    println!("{summa}");
}
