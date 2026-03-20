
use std::io::{self, BufRead, Read, Write};

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

fn read_text() -> io::Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    println!("Текст: {} байт", text.len());

    //binary
    let mut bin = Vec::with_capacity(64 * 1024);
    io::stdin().read_to_end(&mut bin)?;
    println!("Binary: {} байт", bin.len());

    Ok(())
}

fn  summ() -> io::Result<()> {
    
    Ok(())
}

fn main() {
   let _ = read_text(); 
   
}
