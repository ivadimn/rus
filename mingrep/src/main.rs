use std::env;
use mingrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Ошибка при разборе аргументов: {}", err);
        std::process::exit(1);
    });

    println!("Поиск {}", config.query);
    println!("В файле {}", config.filename);

    if let Err(err) = mingrep::run(config) {
        println!("Ошибка в приложении: {}", err);
        std::process::exit(1);
    }

}



