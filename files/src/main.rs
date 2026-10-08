use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

mod rle;
use rle::{coder, decoder};

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
    pub fn save(file_name: &str, data: &mut Vec<Self>) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .append(true) // Устанавливаем режим дозаписи (append)
            .create(true) // Создаем файл, если он не существует
            .open(file_name)?; // Открываем файл


        
        let name: [u8; 512] = [0; 512];
        let email: [u8; 128] = [0; 128];
        for u in data.iter_mut() {
            
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
        //let mut vec_buffer = Vec::new();
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
    
    let arr: Vec<u8> = vec![
        0b00110001,
        0b00010010,
        0b11110011,
        0b11110100,
        0b00000110,
        0b00000111,
        0b00001000,
    ];

    let mut dec: Vec<u8> = Vec::new();

    // let carr: Vec<u8> = vec![
    //     0b00100010,
    //     0b00010010,
    //     0b01000001,
    //     0b00010010,
    //     0b10001111,
    //     0b00000001,
    //     0b10000000,
    // ];

   /* 0000 0010
    0011 0010
    0100 0001
    0001 0010
    0001 0110 */

    // for e in arr.iter()   {
    //     println!("{:08b}", e);        
    // }
    // println!("\n-------------------------------------------------------\n");
    // let code = coder(arr);
    // for e in code.iter()   {
    //     println!("{:08b}", e);        
    // }
    // println!("\n-------------------------------------------------------\n");

    // let decode = decoder(code);

    // for e in decode.iter()   {
    //     println!("{:08b}", e);        
    // }

    let mut position = 0u8;
    let mut byte = 0u8;
    loop {
        println!("\n\nPosition {}", position);
        let aref = &arr;
        for e in aref.into_iter() {
            let mut zero_count = e >> 4;
            let mut one_count = e & 15;

            //let number: u16 = (1 << one_count) - 1;
            let mut can_insert = 8 - position; // количество незаполненных бит в текущем байте

            if can_insert < zero_count {
                dec.push(byte);
                position = 0;
                zero_count -= can_insert;
                can_insert = 8 - position;
                

            } else {
                position += zero_count;
            }

            
            if can_insert <  one_count {
                //
            }
            else {
                let overflow: u16 = 1 << one_count;
                let mut number: u8 = (overflow - 1) as u8; //=3 
                number <<= get_shift(position, one_count);
                println!("count {}, {:08b}", one_count, number);
            }
        }

        position += 1;
        if position == 7 {
            break;
        }   
    }
    

    Ok(())

}


fn get_shift(position: u8, count: u8) -> u8 {
    8 - position - count
}

fn process_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string("http.txt")?;

    match fs::write("output.txt", content) {
        Ok(_) => println!("Файл успешно записан"),
        Err(e) => println!("Ошибка при записи: {}", e),        
    }
    Ok(())

}

