use std::io::{self, Read, Write};

fn weather(t: i32) -> Result<String, String> {
    if t > 30 {
        Ok("Очень жарко".to_string())
    } else if t >= 20 {
        Ok("Тепло".to_string())
    } else if t>= 10 {
        Ok("Прохладно".to_string())
    } else {
       Ok("Холодно".to_string())
    }
}

fn tconvert(input: f64) {
    let unit = "C";
    
    let output = if unit == "C" {
        input * 9.0 / 5.0 + 32.0
    } else {
        (input - 32.0) * 5.0 / 9.0
    }; 
    println!("{input}{unit} = {output:.2} {}", if unit == "C" { "F" } else { "C" });
}

fn get_number() -> i32 {
    let mut s = String::new();
    //print!("Введите число: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    let number = s.trim().parse().unwrap();
    number
}

fn get_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn znak() {
    let number = get_number();
    let zn = if number > 0 {
        "Положительное"
    } else if number < 0 {
        "Отрицательное"
    } else {
        "Ноль"
    };
    println!("{zn}");
}

fn guards() {
    let number = 56;
    match number {
        n if n % 15 == 0 => println!("FizzBuzz"),
        n if n % 3 == 0 => println!("Fizz"),
        n if n % 5 == 0 => println!("Buzz"),
        n => println!("{n}"),
    }
}

fn guards2() {
    #[derive(Debug)]
    enum Token {
        Ident(String),
        Number(i64),
    }
    let token = Token::Ident("Ident".to_string());
    match token {
        ident @ Token::Ident(_) => println!("идентификатор {ident:?}"),
        num @ Token::Number(n) if n < 0 => println!("отрицательное число {num:?}"),
        Token::Number(_) => println!("число")
    }

    let token = Token::Number(89);
    match token {
        ident @ Token::Ident(_) => println!("идентификатор {ident:?}"),
        num @ Token::Number(n) if n < 0 => println!("отрицательное число {num:?}"),
        Token::Number(_) => println!("число")
    }

    let token = Token::Number(-9);
    match token {
        ident @ Token::Ident(_) => println!("идентификатор {ident:?}"),
        num @ Token::Number(n) if n < 0 => println!("отрицательное число {num:?}"),
        Token::Number(_) => println!("число")
    }
}

fn count_down() {
    let mut count: i32 = get_number();
    while  count > 0 {
        print!("{count} ");
        count -= 1;
    }
    println!("Старт!");
}

fn glasnye() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();
    let letters = ['a', 'o', 'e', 'y', 'i', 'j', 'u', 'A', 'O', 'E', 'Y', 'I', 'J', 'U'];

    let mut gl = 0;
    for ch in s.chars() {
        if letters.contains(&ch) {
            gl += 1;
        }
    }
    println!("Количество гласных: {gl}")
}

fn max() {
    let ns = get_numbers();
    let mut m = 0;
    for item in ns.into_iter() {
        if item > m {
            m = item;
        }        
    }
    println!("MAX = {m}");
}

fn sum_before_zero() {
    let ns = get_numbers();
    let mut iter = ns.into_iter();
    let mut sum = 0;
    loop {
        match iter.next() {
            Some(n) => if n != 0 {sum += n;} else {break;},
            None => break, 
        }
    }
    println!("Сумма до нуля: {sum}");
}

fn even() {
    let ns = get_numbers();
    let mut iter = ns.into_iter();
    loop {
        match iter.next() {
            Some(n) => if n % 2 == 0 {
                print!("{n} ");
            },
            None => break, 
        }
    }
}

fn strs() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    for (i, l) in s.lines().into_iter().enumerate() {
        println!("{}: {l}", i + 1);
    }
}

fn fizzbuzz(num: i32) -> String {

    match num {
        n if n % 15 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        n => n.to_string(),
    }
    
}

enum DivisionResult {
    Int(u64),
    WithRemainder(u64, u64),
    DivisionByZero,
}

fn my_divide(a: u64, b: u64) -> DivisionResult {
    if b == 0 {
        DivisionResult::DivisionByZero
    } else if a % b == 0 {
        DivisionResult::Int(a / b)
    } else {
        DivisionResult::WithRemainder(a / b, a % b)
    }   
}

struct Point {
    x: f64,
    y: f64,
}

fn distance(point: Point) -> f64 {
    f64::sqrt(point.x * point.x + point.y * point.y)
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 || n == 2{
        1
    } else {
        let mut f1: u32 = 1;
        let mut f2: u32 = 1;
        for _ in 3..=n {
            let ff = f2;
            f2 = ff + f1;
            f1 = ff;
        }
        f2
    }
}

fn fib_vec(n: u32) -> Vec<u32> {
    let mut fibs: Vec<u32> = Vec::new();
    let mut f1: u32 = 1;
    let mut f2: u32 = 1;
    for i in 0..=n {
        if i == 0 {
           fibs.push(0);
        } else if i == 1 || i == 2{
            fibs.push(1);
        } else {
            let ff = f2;
            f2 = ff + f1;
            f1 = ff;
            fibs.push(f2);
        }
    }
    fibs
}

fn digit_product(n: u32) -> u8 {
    let mut number = n;
    if number == 0 {
        0u8
    } else {
        loop {
            let mut prod: u32 = 1;
            while number > 0 {
                let ost =  number % 10;
                number /= 10;
                if ost == 0 {continue;}
                prod *= ost; 
            }
            if prod < 10 {break prod as u8; }
            number = prod;
        }
    }
    //number
}

fn uniq_digit(s: &str) -> u8 {
    let strs: Vec<char> = s.chars().collect();
    let len = s.chars().count();
    let mut founded: char = '0';
    for ch in s.chars() {
        let mut count = 0;
        let mut j = 0;
        while j < len {
            if strs[j] == ch {
                count += 1;
            }
            j += 1; 
        }
        if count == 1 {
            founded = ch;
            break;
        }
    }    
    founded.to_digit(10).unwrap() as u8
}

fn missing_num(nums: &[i32]) -> i32 {

    let m = nums.iter().max().unwrap();
    
    let mut index :i32 = 0;
    loop {
        if !nums.contains(&index) {
            break index;
        }
        index += 1;
        if index >= *m {
            break index;
        }
    }
}

fn validate_paren(s: &str) -> bool {
    let mut pc = 0;
    let mut stack : Vec<char> = Vec::new();
    let chs = s.chars();
    let mut error: bool = false;
    for ch in chs {
        match ch {
            '{' | '(' | '[' => {
                pc += 1; 
                stack.push(ch);
            },
            '}' => {
                if let Some(p) = stack.pop() {
                    if p == '{' {
                        pc -= 1;
                    } else {
                        error = true;
                        break;
                    }     
                } else {
                    error = true;
                    break;
                }
            },
            ')' => {
                if let Some(p) = stack.pop() {
                    if p == '(' {
                        pc -= 1;
                    } else {
                        error = true;
                        break;
                    }     
                } else {
                    error = true;
                    break;
                } 
            },
            ']' => {
                if let Some(p) = stack.pop() {
                    if p == '[' {
                        pc -= 1;
                    } else {
                        error = true;
                        break;
                    }     
                } else {
                    error = true;
                    break;
                } 
            },
            _ => {},
        };
    }
    (!error) && (pc == 0)
}

fn main() {
    println!("{}", validate_paren(""));
    println!("{}", validate_paren("()"));
    println!("{}", validate_paren("()[]{}"));
    println!("{}", validate_paren("({[]()})"));
    println!("{}", validate_paren("(}"));
    println!("{}", validate_paren("()]"));
    println!("{}", validate_paren("(){"));
    println!("{}", validate_paren("({)}"));
}
