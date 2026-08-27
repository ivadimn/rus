use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

struct Person {
    name: [u8; 512],
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        let mut obj = Self {
            name: [0; 512],
            age 
        };
        
        let tmp =  name.as_bytes();
        for (i, b) in tmp.into_iter().enumerate() {
            obj.name[i] = *b;    
        }
        obj
    }

    pub fn get_name(&self) -> String {
        let vec_name = Vec::from_iter(self.name.into_iter());
        String::from_utf8(vec_name).unwrap()
    }

    pub fn save(file_name: &str, data: Vec<&Self>) -> io::Result<()> {
        let mut file: File; 
        let file_result = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .append(true) // Устанавливаем режим дозаписи (append)
            .create(true) // Создаем файл, если он не существует
            .open(file_name); // Открываем файл

        match file_result {
            Ok(f) => file = f,
            Err(msg) => {
                eprintln!("Ошибка при открытии файла: {}", msg);
                return Err(msg);
            }
        }
        for p in data {
            file.write(&p.name)?;
            file.write(&[p.age])?;
        }
        Ok(())
    } 
}

fn main() -> std::io::Result<()>  {
    
    //let person = Person {name: }
    let person = Person::new(String::from("Пупкин Иван Семенович"), 77);

    let vec_per:  Vec<&Person> = vec![&person];
    
    let result = Person::save("person.data", vec_per);
    match result {
        Ok(_) => println!("Данные успешно записаны"),
        Err(msg) => println!("Ошибка: {}", msg),
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

