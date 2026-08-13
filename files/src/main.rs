use std::io;

mod helpers;
use helpers::*; 

fn main() {
    let full_name = get_full_name("Vadim", "Ivanov");
    println!("Hello from {}", full_name);    
    test_if();
}


fn test_if() {
    let age_to_drive = 16u8;
    println!("Enter persons age: ");

    let myinput: &mut String = &mut String::from("");
    io::stdin().read_line(myinput).unwrap();
    let age  = myinput.trim().parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing driver's because they are old enough!");
    }
    // let myinput: u8 = match myinput.trim().parse() {
    //     Ok(num) => num,
    //     Err(err) => println!("Error: {}", msg),
    // };



}


 