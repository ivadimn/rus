use std::env;
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
    run(config);

}

fn run(config: Config) {

    let contents = fs::read_to_string(config.filename)
                .expect("Что-то пошло не так при чтении файла!");
    println!("С текстом {}", contents);
}

