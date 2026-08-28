use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::MetadataExt;


const RECORD_SIZE: usize = 513;


struct Person {
    name: String,
    age: u8,
}

struct User {
    name: String,
    email: String,
    status: bool,
}

impl User {
    pub fn save(file_name: &str, data: Vec<&Self>) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .append(true) // Устанавливаем режим дозаписи (append)
            .create(true) // Создаем файл, если он не существует
            .open(file_name)?; // Открываем файл


        let name: [u8; 512] = [0; 512];
        let email: [u8; 128] = [0; 128];
        for u in data {
            u.to
            let v = unsafe {
               u.name.as_mut_vec(); 
            }; 
            
        }
        Ok(())
    }
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn get_name(&self) -> &String {
        &self.name
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
        let mut buffer: [u8; 512] = [0; 512];
        for p in data {
            let buf_name = p.name.as_bytes();
            for (i, b) in buf_name.into_iter().enumerate() {
                buffer[i] = *b;
            }

            file.write(&buffer)?;
            file.write(&[p.age])?;
        }
        Ok(())
    } 


    pub fn read(file_name: &str, data: &mut Vec<Self>) -> io::Result<()> {

        //let metadata = fs::metadata(file_name)?;
        //let size = metadata.size();

        let mut buffer: [u8; 513] = [0; 513];
        let mut file = File::open(file_name)?;

        loop {
            let bytes = file.read(&mut buffer)?;
            if bytes == 0 {break;}
            let vec = buffer[0 .. 512].to_vec();
            
            let p = Self {
                name: String::from_utf8(vec).unwrap(),
                age: buffer[512],
            };
            println!("Name: {}", p.name);
            println!("Age: {}", p.age);
        }


        Ok(())
    }
}

fn main() -> std::io::Result<()>  {
    
    //let person = Person {name: }
    let person = Person::new(String::from("Сидоров Сергей Петрович"), 34);

    let mut vec_per:  Vec<Person> = Vec::new();
    Person::read("person.data", &mut vec_per);

    
    // let result = Person::save("person.data", vec_per);
    // match result {
    //     Ok(_) => println!("Данные успешно записаны"),
    //     Err(msg) => println!("Ошибка: {}", msg),
    // }


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

