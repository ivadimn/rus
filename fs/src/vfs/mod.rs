use std::fs::{File, OpenOptions, metadata};
use std::io::{Error, ErrorKind, Read, Write};
use std::mem::size_of;
use fs::get_mime_type;

const HEADER_SIZE: usize = 16;


struct StrData {
    name_len: u16,
    name: String,
}

impl StrData {
    fn get_len(&mut self) -> u16 {
        self.name_len = self.name.len() as u16;
        self.name_len
    }
}


#[derive(Default)]
pub struct Header {
    pub count_items: u64,
    pub start_data_pos: u64,
}

struct Item {
    data_size: u64,
    file_name: StrData,
    mime_type: StrData,
    item_size: u16,
}

#[derive(Default)]
pub struct Vfs {
    header: Header,
    items: Vec<Item>,
    file_name: String,
}


impl Vfs {
    pub fn create(name: &str) -> Result<Self, Error> {

        let _file = File::create_new(name)?; // Создаем файл, если он не существует
            
        let header = Header { start_data_pos: 0, count_items: 0};
        let items: Vec<Item> = Vec::new();

        Ok(Self { header, items, file_name: name.to_string() })
    }

    pub fn open(name: &str) -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .append(true) // Устанавливаем режим дозаписи (append)
            .open(name)?; 

        let header = Vfs::read_header(&mut file)?;
        let items: Vec<Item> = Vec::new();
        
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

    pub fn add(&mut self, file_name: &str) -> Result<(), Error> {

        // let file = OpenOptions::new()
        //                         .read(true)
        //                         .open(file_name)?;
        println!("file name: {}", file_name);
        let attr = metadata(file_name)?;
        let mime_type = get_mime_type(file_name); 

        let mut item = Item {
            data_size: attr.len(),
            file_name: StrData { name_len: 0, name: file_name.to_string(),}, 
            mime_type : StrData { name_len: 0, name: mime_type },
            item_size: 0,
        };
        item.item_size = item.file_name.get_len() + item.mime_type.get_len() + size_of::<u64>() as u16;
        self.items.push(item);
        
        Ok(())
    }

    pub fn show_list(&self) {
        for item in self.items.iter() {
            println!("file name: {}, mime_type {}", item.file_name.name, item.mime_type.name);
        }
    }
    
}

// impl Drop for Vfs {
//     fn drop(&mut self) {
        
//     }
// }
