#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

fn main() {
    let mut opts: Option<i32> = Some(42);
    let mut optn: Option<i32> = None;

    println!("{:?}", optn.ok_or_else(|| "error"));
}