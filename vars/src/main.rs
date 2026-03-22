
use std::io::{self, BufRead, Read, Write};
use std::collections::HashSet;
use std::os::unix::net::UnixDatagram;
//use tokio::io::{self, AsyncBufReadExt};


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

fn read_str() -> io::Result<()> {
    print!("Введите имя: ");
    io::stdout().flush();

    let mut name  = String::new();
    let n = io::stdin().read_line(&mut name)?;

    println!("Прочитана {n} байт.");
    println!("Привет, {}!", name.trim_end());
    Ok(())
}

fn read_strs() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        println!("{} ({} символов)", line, line.len());
    }
    Ok(())
}

//Простейшее чтение всего stdin
fn read_text() -> io::Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    println!("Текст: {} байт", text.len());
    println!("Текст: {} символов", text.chars().count());    
    Ok(())
}

fn read_bytes() -> io::Result<()> {

    //binary
    let mut bin = Vec::with_capacity(64 * 1024);
    io::stdin().read_to_end(&mut bin)?;
    println!("Binary: {} байт", bin.len());
    Ok(())
}

//Построчное чтение с минимумом памяти

fn read_data() -> io::Result<()> {

    let stdin  = io::stdin();       // глобальный объект
    let reader = stdin.lock();      // захватываем поток один раз

    // lines() -> Iterator<Item = Result<String, io::Error>>
    for line in reader.lines() {
        let line = line?;           // здесь возможна ошибка ввода

        if line.eq_ignore_ascii_case("exit") { break; }
        println!("→ {line}");
    }
    Ok(())
}

// fn  summ() -> io::Result<()> {
//     let sum: i64 = io::stdin()
//         .lock()
//         .split(b' ')
//         .map(|chunk| {
//             let bytes = chunk?;
//             let num = std::str::from_utf8(&bytes)?.trim().parse::<i64>()?;
//             Ok::<_, Box<dyn std::error::Error>>(num)
//         })
//         .collect::<Result<Vec<_>, _>>()?
//         .into_iter()
//         .sum();
//     Ok(())
// }

//Уникальные строки — полный пример с подробными комментариями
fn get_unique() -> io::Result<()> {
    // HashSet для проверки «видели ли мы строку».
    let mut seen = HashSet::new();

    // Захватываем stdin и stdout один раз.
    let stdin  = io::stdin();
    let mut out = io::stdout().lock();

    for line_result in stdin.lock().lines() {
        let line = line_result?;
        // insert() вернёт true, если строки ещё не было.
        if seen.insert(line.clone()) {
            writeln!(out, "{line}")?; // выводим без повторов
        }
    }
    Ok(())
}

//Not UTF‑8: читаем бинарный ввод безопасно
fn read_bytes_safe() -> io::Result<()> {
    
    let mut raw = Vec::new();             // просто байты
    io::stdin().read_to_end(&mut raw)?;

    // Пробуем интерпретировать как UTF‑8.
    match String::from_utf8(raw) {
        Ok(text) => println!("строка длиной {}", text.len()),
        Err(e)   => eprintln!("не UTF‑8, {} байт", e.as_bytes().len()),
    }
    Ok(())
}

//Пример с Tokio
// async fn tokio() -> io::Result<()> {

//     let mut stdin = io::BufReader::new(io::stdin());
//     let mut buf = String::new();
//     loop {
//         let n = stdin.read_line(&mut buf).await?;
//         if n == 0 { break; }
//         // обрабатываем buf
//         buf.clear();
//     }

//     Ok(())
// }

fn fmt_sample() {

    let name = "Alice";
    let age = 25;
    let height = 1.68;

    // Примеры использования спецификаторов формата
    println!("Имя: {}, Возраст: {}, Рост: {:.2} м", name, age, height);

    let num1 = 7;
    let num2 = 3;
    // Использование преобразования типов для деления чисел с плавающей точкой
    println!("Результат деления {} на {} = {:.2}", num1, num2, num1 as f32 / num2 as f32);

    // Использование выравнивания и ширины поля
    println!("{:<10} - {:>10}", "Лево", "Право");
    println!("{:^20}", "Центр");

}

fn std1() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("bad numbers");

    if let Some((a, b)) = numbers.split_once(' ') {
        let a: f64 = a.trim().parse().expect("bad number");
        let b: f64 = b.trim().parse().expect("bad number");
        println!("{:.2}", a / b);
    }
}

fn std2() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("bad numbers");
    let date: Vec<u32> = numbers.split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let d = date[0];
    let m = date[1];
    let y = date[2];
    println!("{d:02}/{m:02}/{y}");

}

fn main() {
   std2();
   
}
