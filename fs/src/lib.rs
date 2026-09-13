use std::io::{self, Write};



pub fn get_num(prompt: &str, max_item: usize) -> usize {

    let mut input = String::new();
    let mut num: usize;
    loop {
        print!("{}:> ", prompt);
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }
        
        num = input.trim().parse().unwrap_or(0);
        if num == 0 || num > max_item { 
           println!("Неправильный пунк меню...");
           continue; 
        }
        else {
            break;
        }
    }
    num
}

pub fn get_str(prompt: &str) -> String {
    let mut input = String::new();
    
    loop {
        print!("{}:> ", prompt);
        io::stdout().flush().unwrap();    
        input.clear();

        if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }

        let input = input.trim().replace("/\\", "");
        if input.len() == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }
        else {
            break;
        }
    }
    input
}