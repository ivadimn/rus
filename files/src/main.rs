use std::fs::{self, Metadata};
use std::io::{self, Write};
use std::path::Path;

fn main() -> std::io::Result<()>  {
    // let content = fs::read_to_string("http.txt");
    // match content {
    //     Ok(s) => println!("Содержимое файла: \n{}", s),
    //     Err(err) => println!("Ошибка чтения файла: {}", err),
    // }
    // let bytes = fs::read("http.txt")?;
    // println!("Прочитано {} байт", bytes.len());

    // fs::write("output.txt", "Привет мир....")?;

    // let mut file = fs::OpenOptions::new()
    //     .append(true)
    //     .create(true)
    //     .open("log.txt")?;
    // writeln!(file, "Новая запись в логе")?;

    //fs::create_dir("new_folder")?;

    //fs::create_dir_all("path/to/nested/folder")?;

    let entries = fs::read_dir(".")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let file_type = if metadata.is_dir() {
            "Директория"
        } else if metadata.is_file() {
            "Файл"
        } else {
            "Другое"
        };
        println!("{}: {}", path.display(), file_type);
    }

    if let Err(e) = process_file("http.txt") {
        eprintln!("Ошибка: {}", e)
    }
   

    Ok(())
    
}

fn process_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string("http.txt")?;

    match fs::write("output.txt", content) {
        Ok(_) => println!("Файл успешно записан"),
        Err(e) => println!("Ошибка при записи: {}", e),        
    }
    Ok(())

}

