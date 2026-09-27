use std::fs::{File, OpenOptions, metadata};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::vec;
use std::mem::size_of;

const SIZE_STEP: usize = 2;
const SIZE_HEADER: usize = 16;
const SIZE_DATA_SIZE: usize = 8;


#[derive(Debug)]
pub struct Item {
    item_size: u16,
    file_name_len: u16,
    file_name: String,
    mime_type_len: u16,
    mime_type: String,
    file_pos: u64,
    file_size: u64,
}

impl Item {
    pub fn item_to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.extend(self.item_size.to_ne_bytes());
        buffer.extend(self.file_name_len.to_ne_bytes());
        buffer.extend(self.file_name.as_bytes());
        buffer.extend(self.mime_type_len.to_ne_bytes());
        buffer.extend(self.mime_type.as_bytes());
        buffer.extend(self.file_pos.to_ne_bytes());
        buffer.extend(self.file_size.to_ne_bytes());
        buffer
    }
}

pub struct Vfs {
    pub count_items: u64,
    pub header_size: u64,
    file_name: String,
    pub items: Vec<Item>,
}

impl Vfs {
    pub fn create(file_name: &str) -> Self {
        Self { count_items: 0, header_size: 0, 
            file_name: file_name.to_string(), items: Vec::new() }    
    }

    //создание нового элемента архива и добавление его в список
    pub fn add(&mut self, file_name: &str) {
        //здесь вставить обработку ошибки metadata
        let attr = metadata(file_name).unwrap();
        
        let file_name_len = file_name.len() as u16;
        let mime_type = get_mime_type(file_name);
        let mime_type_len = mime_type.len() as u16;
        let file_pos = 0;
        let file_size = attr.len();
        let item_size = (SIZE_STEP * 2) as u16 
                        + file_name_len + mime_type_len 
                        + (SIZE_DATA_SIZE * 2) as u16; 
        let item = Item {item_size, 
                               file_name_len, 
                               file_name: file_name.to_string(),
                               mime_type_len, mime_type, 
                               file_pos, file_size };
        
        self.items.push(item);
        self.header_size += item_size as u64; //увеличиваем общий размер заголовка
        
        self.count_items += 1;

    }

    pub fn save(&mut self) -> io::Result<()>{

        let mut header_buffer: Vec<u8> = Vec::new(); 
        let mut file = File::create("test.arc")?;
        
        let buf = self.count_items.to_ne_bytes();
        let _writed = file.write(&buf)?;            //зописываем колическтов файлов
        let buf = self.header_size.to_ne_bytes();
        let _writed = file.write(&buf)?;            //записываем размер всего оглавления
        self.calculate_positions();

        //формируем байтовый буфер оглавления и записываем его в файл
        for item in self.items.iter() {
            header_buffer.extend(item.item_to_bytes());
        }
        let _writed = file.write(&header_buffer)?;
       
        
        Ok(())
    }

    pub fn open(file_name: &str) -> io::Result<Self> {

        let mut file = File::open(file_name)?;
        let mut items: Vec<Item> = Vec::new();

        //читаем количество элементов архива
        let mut buf = [0u8; SIZE_DATA_SIZE];
        let readed = file.read(&mut buf)?;
        let count_items = u64::from_ne_bytes(buf);

        //читаем общий размер оглавления
        let mut buf = [0u8; SIZE_DATA_SIZE];
        let readed = file.read(&mut buf)?;
        let header_size = u64::from_ne_bytes(buf);

        for _ in 0 .. count_items {
            let mut pos: usize = 0;
            let mut size_buf = [0u8; SIZE_STEP];
            //читаем из файла 2 байта: размер Item
            let _readed = file.read(&mut size_buf)?;
            
            let item_size = u16::from_ne_bytes(size_buf); //получаем размер Itme

            //читаем из файла item_size байт это буфер для Item
            let mut item_buffer: Vec<u8> = vec![0u8; item_size as usize];
            let _readed = file.read(&mut item_buffer).unwrap();
            
            //получаем длину имени файла
            let size_buf: &[u8; 2] = item_buffer[pos .. pos + SIZE_STEP].as_array().unwrap();;
            let file_name_len = u16::from_ne_bytes(*size_buf);
            pos += SIZE_STEP;

            //получаем имя файла
            let mut name_buf: Vec<u8> = Vec::new();
            name_buf.extend(item_buffer[pos .. pos + file_name_len as usize].iter());  
            let file_name = String::from_utf8(name_buf).unwrap();
            pos += file_name_len as usize;

            // получаем длинк mime type
            let size_buf: &[u8; 2] = item_buffer[pos .. pos + SIZE_STEP].as_array().unwrap();
            let mime_type_len = u16::from_ne_bytes(*size_buf);
            pos += SIZE_STEP;

            // получаем сам mine type
            let mut name_buf: Vec<u8> = Vec::new();
            name_buf.extend(item_buffer[pos .. pos + mime_type_len as usize].iter());  
            let mime_type = String::from_utf8(name_buf).unwrap();
            pos += mime_type_len as usize;

            // получаем позицию начала файла в архиве
            let size_buf: &[u8; 8] = item_buffer[pos .. pos + SIZE_DATA_SIZE].as_array().unwrap();
            let file_pos = u64::from_ne_bytes(*size_buf);
            pos += SIZE_DATA_SIZE;
        
            // получаем размер самого файла
            let size_buf: &[u8; 8] = item_buffer[pos .. pos + SIZE_DATA_SIZE].as_array().unwrap();
            let file_size = u64::from_ne_bytes(*size_buf);

            let item = Item {item_size, file_name_len, file_name, mime_type_len, mime_type, 
                                file_pos, file_size};
            items.push(item);                    
            
        }

        Ok(Self {count_items, header_size, file_name: file_name.to_string(), items })


    }

    fn calculate_positions(&mut self) {
        let mut pos: u64 = SIZE_HEADER as u64 + self.header_size;
        for item in self.items.iter_mut()  {
            item.file_pos = pos;
            pos += item.file_size;
        }
    }

}


pub fn create_item(file_name: &str) -> Item {
    
    let attr = metadata(file_name).unwrap();
    let file_name_len = file_name.len() as u16;
    let mime_type = get_mime_type(file_name);
    let mime_type_len = mime_type.len() as u16;
    let file_pos = 0;
    let file_size = attr.len();
    let item_size = (SIZE_STEP * 2) as u16 
                        + file_name_len + mime_type_len 
                        + (SIZE_DATA_SIZE * 2) as u16; 
    Item {item_size, 
            file_name_len, 
            file_name: file_name.to_string(),
        mime_type_len, mime_type, file_pos, file_size }
}

pub fn item_to_bytes(item: &Item) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    buffer.extend(item.item_size.to_ne_bytes());
    buffer.extend(item.file_name_len.to_ne_bytes());
    buffer.extend(item.file_name.as_bytes());
    buffer.extend(item.mime_type_len.to_ne_bytes());
    buffer.extend(item.mime_type.as_bytes());
    buffer.extend(item.file_pos.to_ne_bytes());
    buffer.extend(item.file_size.to_ne_bytes());
    buffer
}

pub fn get_metadata(file_name: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    let attr = metadata(file_name).unwrap();
    let name_len = file_name.len() as u16;
    buffer.extend(name_len.to_ne_bytes());
    buffer.extend(file_name.as_bytes());
    buffer.extend(attr.len().to_ne_bytes());
    buffer
}

pub fn read_items(file_name: &str) {
    let mut file = OpenOptions::new() 
                    .read(true)
                    .open(file_name).unwrap();

    loop {

        let mut buf_name_len = [0u8; 2];
        let readed = file.read(&mut buf_name_len).unwrap();
        if readed == 0 {
            break;
        }
        let len: u16 = u16::from_ne_bytes(buf_name_len);
        let mut buf_name = vec![0u8; len as usize];
        file.read_exact(&mut buf_name).unwrap();
        let name = String::from_utf8(buf_name).unwrap();
        let mut buf_size = [0u8; 8];
        let _readed = file.read(&mut buf_size).unwrap();
        let size: u64 = u64::from_ne_bytes(buf_size);

        println!("длина имени файла: {} Имя файла: {} Размер файла {}", len, name, size)
    }

}

pub fn read_items1(file_name: &str) {
    let mut file = OpenOptions::new() 
                    .read(true)
                    .open(file_name).unwrap();

    loop {
        let mut pos: usize = 0;
        let mut size_buf = [0u8; SIZE_STEP];
        //читаем из файла 2 байта: размер Item
        let readed = file.read(&mut size_buf).unwrap();
        if readed == 0 {
            break;
        }
        let item_size = u16::from_ne_bytes(size_buf); //получаем размер Itme

        //читаем из файла item_size байт это буфер для Item
        let mut item_buffer: Vec<u8> = vec![0u8; item_size as usize];
        let readed = file.read(&mut item_buffer).unwrap();
        if readed == 0 {
            break;
        }
        //получаем длину имени файла
        let size_buf: &[u8; 2] = item_buffer[pos .. pos + SIZE_STEP].as_array().unwrap();
        let file_name_len = u16::from_ne_bytes(*size_buf);
        pos += SIZE_STEP;

        //получаем имя файла
        let mut name_buf: Vec<u8> = Vec::new();
        name_buf.extend(item_buffer[pos .. pos + file_name_len as usize].iter());  
        let file_name = String::from_utf8(name_buf).unwrap();
        pos += file_name_len as usize;

        // получаем длинк mime type
        let size_buf: &[u8; 2] = item_buffer[pos .. pos + SIZE_STEP].as_array().unwrap();
        let mime_type_len = u16::from_ne_bytes(*size_buf);
        pos += SIZE_STEP;

        // получаем сам mine type
        let mut name_buf: Vec<u8> = Vec::new();
        name_buf.extend(item_buffer[pos .. pos + mime_type_len as usize].iter());  
        let mime_type = String::from_utf8(name_buf).unwrap();
        pos += mime_type_len as usize;

        // получаем позицию начала файла в архиве
        let size_buf: &[u8; 8] = item_buffer[pos .. pos + SIZE_DATA_SIZE].as_array().unwrap();
        let file_pos = u64::from_ne_bytes(*size_buf);
        pos += SIZE_DATA_SIZE;
        
        // получаем размер самого файла
        let size_buf: &[u8; 8] = item_buffer[pos .. pos + SIZE_DATA_SIZE].as_array().unwrap();
        let file_size = u64::from_ne_bytes(*size_buf);

        let item = Item {item_size, file_name_len, file_name, mime_type_len, mime_type, 
                                file_pos, file_size};
        println!("{:?}", item)
    }

}

pub fn read_all_items(file_name: &str) {

    let attr = metadata(file_name).unwrap();
    println!("Размер файла {}", attr.len());

    let mut file = OpenOptions::new() 
                    .read(true)
                    .open(file_name).unwrap();

    let mut buffer: Vec<u8> = Vec::new();

    let readed = file.read_to_end(&mut buffer).unwrap();
    println!("Прочитано {} байт", readed);
    
    println!("Buffer: {:?}", buffer);
    // loop {

    //     let mut buf_name_len = [0u8; 2];
    //     let readed = file.read(&mut buf_name_len).unwrap();
    //     if readed == 0 {
    //         break;
    //     }
    //     let len: u16 = u16::from_ne_bytes(buf_name_len);
    //     let mut buf_name = vec![0u8; len as usize];
    //     file.read_exact(&mut buf_name).unwrap();
    //     let name = String::from_utf8(buf_name).unwrap();
    //     let mut buf_size = [0u8; 8];
    //     let _readed = file.read(&mut buf_size).unwrap();
    //     let size: u64 = u64::from_ne_bytes(buf_size);

    //     println!("длина имени файла: {} Имя файла: {} Размер файла {}", len, name, size)
    // }

}

pub fn get_mime_type(file_name: &str) -> String {

    let ext: Vec<&str>= file_name.split(".").collect();
    let ext = ext.last().unwrap();

    match *ext {
        "txt" => "application/text".to_string(),
        "html" => "application/text".to_string(),
        "htm" => "application/text".to_string(),
        "sh" => "application/bash".to_string(),
        "docx" => "application/document".to_string(),
        "xlsx" => "application/document".to_string(),
        "pptx" => "application/document".to_string(),
        "doc" => "application/document".to_string(),
        "xls" => "application/document".to_string(),
        "ppt" => "application/document".to_string(),
        "docs" => "application/document".to_string(),
        "jpg" => "application/image".to_string(),
        "jpeg" => "application/image".to_string(),
        "png" => "application/image".to_string(),
        "bmp" => "application/image".to_string(),
        "webp" => "application/image".to_string(),
        "tiff" => "application/image".to_string(),
        "gif" => "application/image".to_string(),
        "avi" => "application/video".to_string(),
        "mp4" => "application/video".to_string(),
        "mkv" => "application/video".to_string(),
        "mp3" => "application/audio".to_string(),
        "wav" => "application/audio".to_string(),
        "pdf" => "application/document".to_string(),
        "epub" => "application/document".to_string(),
        "zip" => "application/archive".to_string(),
        "7z" => "application/archive".to_string(),
        "rar" => "application/archive".to_string(),
        "tar" => "application/archive".to_string(),
        _ => "application/bin".to_string(),
    }
}