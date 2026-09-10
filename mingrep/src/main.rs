use std::env;
use std::error::Error;
use std::fs;


struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return  Err("Недостаточно аргументов!!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self {query, filename})
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Ошибка при разборе аргументов: {}", err);
        std::process::exit(1);
    });

    println!("Поиск {}", config.query);
    println!("В файле {}", config.filename);

    if let Err(err) = run(config) {
        println!("Ошибка в приложении: {}", err);
        std::process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    println!("С текстом {}", contents);

    Ok(())
}

