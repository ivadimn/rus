use std::fs::{File, OpenOptions, metadata};
use std::io::{Error, ErrorKind, Read, Write};
use std::mem::size_of;
use fs::get_mime_type;

const HEADER_SIZE: usize = 16;


#[derive(Default)]
pub struct Header {
    pub count_items: u64,
    pub start_data_pos: u64,
}

struct Item {
    item_size: u16,
    data_size: u64,
    file_name: String,
    mime_type: String,
}

impl Item {
    fn to_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.extend(self.item_size.to_ne_bytes());
        v.extend(self.data_size.to_ne_bytes());
        v.extend(self.file_name.len().to_ne_bytes());
        v.extend(self.file_name.bytes());
        v.extend(self.mime_type.len().to_ne_bytes());
        v.extend(self.mime_type.bytes());
        v
    }
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
            
        let header = Header { start_data_pos: (size_of::<u64>() * 2) as u64, 
                                    count_items: 0};
        let items: Vec<Item> = Vec::new();

        Ok(Self { header, items, file_name: name.to_string() })
    }

    pub fn open(name: &str) -> Result<Self, Error> {
        
        let mut buffer2: [u8; 2] = [0; 2];
        let mut file = OpenOptions::new()
            .read(true)  // Разрешаем запись
            .open(name)?; 
         
        let header = Vfs::read_header(&mut file)?;
        let items: Vec<Item> = Vec::new();
        for i in 0 .. header.count_items {
            let _readed = file.read(&mut buffer2).unwrap();
            let item_size = u16::from_ne_bytes(buffer2);
            println!("item size: {}", item_size);
        }
        
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

            let bytes: &[u8; 8] = buffer[8 .. ].as_array().unwrap();
            start_data_pos = u64::from_ne_bytes(*bytes);
        }

        println!("Items count: {}, data position {}", count_items, start_data_pos);

        Ok(Header {count_items, start_data_pos})
    }

    fn read_item(item_size: u16, file: &mut File) {
        
        let mut buf = vec![0u8; item_size as usize]; 
        let _readed = file.read_exact(&mut buf); //прочитали item
        //парсим item
        let bytes: &[u8; 8] = buf[ .. 8].as_array().unwrap();
        let data_size = u64::from_ne_bytes(*bytes); //получили размер данных
        let bytes: &[u8; 2] = buf[8..10].as_array().unwrap();
        let name_len = u16::from_ne_bytes(*bytes);
        let mut name: Vec<u8> =  Vec::new();
        name.extend(buf[10 .. name_len as usize].iter());
        let fname = String::from_utf8(name); 


        
        println!("raw item {:?}", buf);
    }

    pub fn save(&mut self) -> Result<(), Error> {

        let mut buffer: Vec<u8> = Vec::new();
        let mut file = OpenOptions::new()
            .write(true)  // Разрешаем запись
            .open(&self.file_name)?;

        let mut vheader: Vec<u8> = Vec::new();
        vheader.extend(self.header.count_items.to_ne_bytes());
        vheader.extend(self.header.start_data_pos.to_ne_bytes());

        println!("vheader: {:?}", vheader);
        let _writed = file.write(&vheader)?;
        

        for item in self.items.iter() {
            buffer.extend(item.to_bytes());
        }

        let writed = file.write(&buffer)?;
        println!("Записано {} байт", writed);
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
            item_size: 0,
            data_size: attr.len(),
            file_name: file_name.to_string(), 
            mime_type : mime_type,
        };

        item.item_size = item.file_name.len() as u16 
                        + item.mime_type.len() as u16
                        + size_of::<u64>() as u16
                        + (size_of::<u16>() * 2) as u16;

        self.header.count_items += 1;
        self.header.start_data_pos += item.item_size as u64 + attr.len();
        self.items.push(item);
        
        
        Ok(())
    }

    pub fn show_list(&self) {
        for item in self.items.iter() {
            println!("file name: {}, mime_type {}", item.file_name, item.mime_type);
        }
    }
    
}

// impl Drop for Vfs {
//     fn drop(&mut self) {
        
//     }
// }
