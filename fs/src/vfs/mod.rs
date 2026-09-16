use std::fs::{File, OpenOptions, metadata};
use std::io::{Error, ErrorKind, Read, Write};

const HEADER_SIZE: usize = 16;



#[derive(Default)]
pub struct Header {
    pub count_items: u64,
    pub start_data_pos: u64,
}

struct Item {
    data_size: u64,
    item_size: u16,
    mime_type_size: u8,
    file_name: String,
    mime_type: String,
}

#[derive(Default)]
pub struct Vfs {
    header: Header,
    items: Vec<u8>,
    file_name: String,
}


impl Vfs {
    pub fn create(name: &str) -> Result<Self, Error> {

        let _file = File::create_new(name)?; // Создаем файл, если он не существует
            
        let header = Header { start_data_pos: 0, count_items: 0};
        let items: Vec<u8> = Vec::new();

        Ok(Self { header, items, file_name: name.to_string() })
    }

    pub fn open(name: &str) -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .append(true) // Устанавливаем режим дозаписи (append)
            .open(name)?; 

        let header = Vfs::read_header(&mut file)?;
        let items: Vec<u8> = Vec::new();
        
        Ok(Self {header, items, file_name: name.to_string()})
    }

    fn read_header(file: &mut File) -> Result<Header, Error> {

        let mut buffer: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
        let count_items: u64;
        let start_data_pos: u64;

        let readed = file.read(&mut buffer)?;


        if readed != HEADER_SIZE {
            return Err(Error::new(ErrorKind::InvalidInput, 
                "Не удалось прочитать заголовок Vfs".to_string()));
        }
        else {

            let bytes: &[u8; 8] = buffer[ .. 8].as_array().unwrap();
            count_items = u64::from_ne_bytes(*bytes);

            let bytes: &[u8; 8] = buffer[ .. 8].as_array().unwrap();
            start_data_pos = u64::from_ne_bytes(*bytes);
        }

        Ok(Header {count_items, start_data_pos})
    }

    pub fn save(&mut self) -> Result<(), Error>{

        let mut file = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .append(true) // Устанавливаем режим дозаписи (append)
            .open(&self.file_name)?;

        let bytes = self.header.count_items.to_ne_bytes();
        let writed = file.write(&bytes)?;
        println!("Writed len: {}", writed);
        let bytes = self.header.start_data_pos.to_ne_bytes();
        let writed = file.write(&bytes)?;
        println!("Writed len: {}", writed);

        Ok(())
    }
    
}

// impl Drop for Vfs {
//     fn drop(&mut self) {
        
//     }
// }
