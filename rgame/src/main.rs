use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("Секретное число равно: {}", secret_number);

    loop {
        println!("Пожалуйста, введите свою догадку.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не получилось прочитать строку");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Вы загадли: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое число"),
            Ordering::Greater => println!("Слишком большое число"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            } 
        }    
    }

}
