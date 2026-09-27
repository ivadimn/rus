use std::collections::HashMap;
use std::fs::{File, metadata};
use std::io::{self, Read, Write, Seek, SeekFrom, BufReader};
use std::vec;
use fs::{get_mime_type, write_buffered, write_full, read_full, read_buffered};

const SIZE_STEP: usize = 2;
const SIZE_HEADER: usize = 8;
const SIZE_DATA_SIZE: usize = 8;
const SIZE_CONST_PART: usize = (SIZE_STEP * 3) + (SIZE_DATA_SIZE * 2);
const MAX_BUFFER_SIZE: u64 = 65535;


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

#[derive(Default)]
pub struct Vfs {
    pub count_items: u64,
    //pub header_size: u64,
    pub file_name: String,
    pub items: Vec<Item>,
    indexes: HashMap<String, usize>,
}

impl Vfs {
    pub fn create(file_name: &str) -> Self {
        Self { count_items: 0, file_name: file_name.to_string(), 
            items: Vec::new(), indexes: HashMap::new()}    
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
        
        println!("Подсчитанные байты {} ", item.item_to_bytes().len());
        //self.items.insert(file_name.to_string(),item);
        self.items.push(item);
        self.indexes.insert(file_name.to_string(), self.items.len() - 1);

        //self.header_size += item_size as u64 + SIZE_STEP as u64; //увеличиваем общий размер заголовка
        self.count_items += 1;
        println!("Добавили {}, item_size {}, file_size {}", file_name, item_size, file_size);
        

    }

    pub fn save(&mut self) -> io::Result<()>{

        let mut header_buffer: Vec<u8> = Vec::new(); 
        let mut file = File::create(&self.file_name)?;
        
        let buf = self.count_items.to_ne_bytes();
        let _writed = file.write(&buf)?;            //зописываем колическтов файлов

        println!("Записано {} элементов заголовка {} байт", self.count_items, SIZE_DATA_SIZE);

        //let buf = self.header_size.to_ne_bytes();
        //let _writed = file.write(&buf)?;            //записываем размер всего оглавления

        //println!("Записано {} байт оглавления writed {}", self.header_size, _writed);

        //формируем байтовый буфер оглавления и записываем его в файл и вычисляем его размер
        let mut header_size = 0u64;
        for item in self.items.iter() {
            header_size += SIZE_CONST_PART as u64 + (item.file_name_len + item.mime_type_len) as u64;
        }
        self.calculate_positions(header_size + SIZE_HEADER as u64);

        for item in self.items.iter() {
            let bytes = item.item_to_bytes(); 
            header_buffer.extend(bytes);
        }

        println!("Сравниваем размеры {} = {}", header_size, header_buffer.len());

        let _writed = file.write(&header_buffer)?;

        for item in self.items.iter() {
            if item.file_size > MAX_BUFFER_SIZE {
                write_buffered(&item.file_name, &mut file)?;    
            }
            else {
                write_full(&item.file_name, &mut file)?;
            }
            //write_full(&item.file_name, &mut file)?;

        }       
        
        Ok(())
    }

    pub fn open(file_name: &str) -> io::Result<Self> {

        let mut file = File::open(file_name)?;
        let mut items: Vec<Item> = Vec::new();
        let mut indexes: HashMap<String, usize> = HashMap::new();

        //читаем количество элементов архива
        let mut buf = [0u8; SIZE_DATA_SIZE];
        let _readed = file.read(&mut buf)?;
        let count_items = u64::from_ne_bytes(buf);

        println!("\nCount items {}", count_items);
        
        //читаем общий размер оглавления
        //let mut buf = [0u8; SIZE_DATA_SIZE];
        //let _readed = file.read(&mut buf)?;
        //let header_size = u64::from_ne_bytes(buf);

        //println!("\nHeader size {}", header_size);

        for _ in 0 .. count_items {
            let mut pos: usize = 0;
            let mut size_buf = [0u8; SIZE_STEP];
            //читаем из файла 2 байта: размер Item
            let _readed = file.read(&mut size_buf)?;
            
            let item_size = u16::from_ne_bytes(size_buf); //получаем размер Itme

            //println!("Item size {}", item_size);

            //читаем из файла item_size байт это буфер для Item
            let mut item_buffer: Vec<u8> = vec![0u8; item_size as usize];
            let _readed = file.read(&mut item_buffer).unwrap();
            
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

           
            println!("\nFile name {}, file pos {} file size {}", file_name, file_pos, file_size);                    
            let item = Item {item_size, file_name_len, file_name: file_name.clone(), mime_type_len, mime_type, 
                                file_pos, file_size};

            items.push(item);  
            indexes.insert(file_name, items.len() - 1);                  
            
        }

        Ok(Self {count_items, file_name: file_name.to_string(), items, indexes })
    }

    pub fn take_item(&self, file_name: &str) ->  io::Result<()> {

        if let Some(index) = self.indexes.get(file_name) {
            let item = &self.items[*index];
            self.take_save(&self.file_name, &item)?;
        }

        Ok(())
    }

    pub fn list_items(&self) {
        println!("\nСостав архива:");
        for item  in self.items.iter() {
            println!("{} - {}, размер {}, позиция {}", 
                item.file_name, item.mime_type, item.file_size, item.file_pos);
        }
    }
    
    
    fn calculate_positions(&mut self, header_size: u64) {
        println!("\nРазмер заголовка {}", header_size);
        let mut pos: u64 = header_size;
        for item in self.items.iter_mut()  {
            println!("Файл {} позиция {}", item.file_name, pos);
            item.file_pos = pos;
            pos += item.file_size;
        }
    }

    fn take_save(&self, archive: &str, item: &Item) -> io::Result<()> {

        let mut farch = File::open(archive)?;

        if item.file_size < MAX_BUFFER_SIZE {
            farch.seek(SeekFrom::Start(item.file_pos))?;
            read_full(&mut farch, &item.file_name, item.file_size)?;    
        }
        else {
            let mut breader = BufReader::new(farch);
            breader.seek(SeekFrom::Start(item.file_pos))?;
            read_buffered(&mut breader, &item.file_name, item.file_size)?;
        }
                
        Ok(())
    }

}

