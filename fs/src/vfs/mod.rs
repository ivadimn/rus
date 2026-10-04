use std::collections::HashMap;
use std::fs::{File, Metadata, metadata};
use std::io::{self, BufReader, Error, Seek, SeekFrom};
use fs::{get_mime_type, read_full, read_buffered};
use crate::fileoper;
use fileoper::FileOper;

const SIZE_STEP: usize = 2;
const SIZE_COUNT_ITEMS: usize = 8;
const SIZE_DATA_SIZE: usize = 8;
const SIZE_HEADER_SIZE: usize = 8;
//const SIZE_CONST_PART: usize = (SIZE_STEP * 3) + (SIZE_DATA_SIZE * 2);
const MAX_BUFFER_SIZE: u64 = 65536;



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

    pub fn item_from_bytes(item_size: u16, bytes: &Vec<u8>) -> Self {
        //получаем длину имени файла
        let mut pos = 0usize;
        let size_buf: &[u8; 2] = bytes[pos .. pos + SIZE_STEP].as_array().unwrap();
        let file_name_len = u16::from_ne_bytes(*size_buf);
        pos += SIZE_STEP;

        //получаем имя файла
        let mut name_buf: Vec<u8> = Vec::new();
        name_buf.extend(bytes[pos .. pos + file_name_len as usize].iter());  
        let file_name = String::from_utf8(name_buf).unwrap();
        pos += file_name_len as usize;

        // получаем длинy mime type
        let size_buf: &[u8; 2] = bytes[pos .. pos + SIZE_STEP].as_array().unwrap();
        let mime_type_len = u16::from_ne_bytes(*size_buf);
        pos += SIZE_STEP;

        // получаем сам mine type
        let mut name_buf: Vec<u8> = Vec::new();
        name_buf.extend(bytes[pos .. pos + mime_type_len as usize].iter());  
        let mime_type = String::from_utf8(name_buf).unwrap();
        pos += mime_type_len as usize;

            // получаем позицию начала файла в архиве
        let size_buf: &[u8; 8] = bytes[pos .. pos + SIZE_DATA_SIZE].as_array().unwrap();
        let file_pos = u64::from_ne_bytes(*size_buf);
            
        pos += SIZE_DATA_SIZE;
        
            // получаем размер самого файла
        let size_buf: &[u8; 8] = bytes[pos .. pos + SIZE_DATA_SIZE].as_array().unwrap();
        let file_size = u64::from_ne_bytes(*size_buf);

        Item {item_size, file_name_len, file_name: file_name.clone(), mime_type_len, mime_type, 
                                file_pos, file_size,}

    }
}


pub struct Vfs {
    pub count_items: u64,
    //pub header_size: u64,
    pub file_name: String,
    pub items: Vec<Item>,
    indexes: HashMap<String, usize>,
    tmp_file_name: String,
}

impl Vfs {
    pub fn create(file_name: &str) -> Self {
        let tmp_file_name = format!("~{}", file_name);
        Self { count_items: 0, file_name: file_name.to_string(), 
            items: Vec::new(), indexes: HashMap::new(), 
            tmp_file_name, }
    }

    //создание нового элемента архива и добавление его в список
    pub fn add(&mut self, file_name: &str) -> io::Result<()>{
        //здесь вставить обработку ошибки metadata

        let attr: Metadata; 
        let result = metadata(file_name);

        match result {
            Ok(m) => attr = m,
            Err(err) =>  return Err(err),
        }
        
        let file_name_len = file_name.len() as u16;
        let mime_type = get_mime_type(file_name);
        let mime_type_len = mime_type.len() as u16;
        let mut file_pos = 0;
        let file_size = attr.len();
        let item_size = (SIZE_STEP * 2) as u16 
                        + file_name_len + mime_type_len 
                        + (SIZE_DATA_SIZE * 2) as u16; 

        //добавляем содержимое файла в конец архива (временный файл)
        FileOper::copy_to(&self.tmp_file_name, file_name, file_size)?;
        if self.items.len() > 0 {
            let item: &Item = &self.items[self.items.len() - 1];
            file_pos = item.file_pos + item.file_size;
        }

        let item = Item {item_size, 
                               file_name_len, 
                               file_name: file_name.to_string(),
                               mime_type_len, mime_type, 
                               file_pos, file_size, };
        
        println!("Подсчитанные байты {} ", item.item_to_bytes().len());
        
        
        self.items.push(item);
        self.indexes.insert(file_name.to_string(), self.items.len() - 1);

        self.count_items += 1;
        println!("Добавили {}, item_size {}, file_size {}", file_name, item_size, file_size);
        Ok(())

    }

    pub fn save(&mut self) -> io::Result<()>{

        let mut header_buffer: Vec<u8> = Vec::new(); 
        let mut items_buffer:  Vec<u8> = Vec::new(); 
        
        //формируем байтовый буфер оглавления 
        //позиции начала файлов рассчитываются при добавлении файла в архив
        //добавляем в header_buffer элементы оглавления
        for item in self.items.iter() {
            let bytes = item.item_to_bytes(); 
            println!("Длина item {}: {}", item.file_name, bytes.len());
            items_buffer.extend(bytes);
        }

        println!("чистая длина оглавления {}", items_buffer.len());


        let buf = (items_buffer.len()  + SIZE_COUNT_ITEMS * 2).to_ne_bytes();
        header_buffer.extend(buf);

        println!("записали в первые 8 байт item_buffer.len {} + {} = {}", 
            items_buffer.len(), SIZE_COUNT_ITEMS, items_buffer.len() + SIZE_COUNT_ITEMS);

        println!("размер оглавления {}", items_buffer.len());
        let buf = self.count_items.to_ne_bytes();
        header_buffer.extend(buf);

        println!("записали во вторые 8 байт items count {}", self.count_items);
        
        header_buffer.extend(items_buffer);
        /*
        * Сформировали буфер загловка:
        * первые 8 байт - общий размер заголовка с количеством файлов в оглавлении
        * вторые 8 байт количество файлов в заголовке
        * далее в буфере записано оглавление
        */
        FileOper::copy_save(&self.file_name, &self.tmp_file_name, header_buffer)?;

        std::fs::remove_file(&self.tmp_file_name)

    }

    // открыть архив, прочитать оглавление и создать временный файл с держимым
    pub fn open(file_name: &str) -> io::Result<Self> {

        //let mut file = File::open(file_name)?;
        let mut items: Vec<Item> = Vec::new();
        let mut indexes: HashMap<String, usize> = HashMap::new();
        let mut pos = 0usize;
        let mut data_size = 0u64;

        //читаем общий размер заголовка
        let header = FileOper::read_header(file_name)?;

        let buf: &[u8; 8] = &header[pos .. SIZE_HEADER_SIZE].as_array().unwrap();
        let header_size = u64::from_ne_bytes(*buf);
        pos += SIZE_HEADER_SIZE;
        
        let buf: &[u8; 8] = &header[pos .. pos + SIZE_COUNT_ITEMS].as_array().unwrap();
        let count_items = u64::from_ne_bytes(*buf);
        pos += SIZE_COUNT_ITEMS;

        println!("\nCount items {}", count_items);
        
        for _ in 0 .. count_items {
           
            let size_buf: &[u8; SIZE_STEP] = header[pos .. pos + SIZE_STEP].as_array().unwrap();
            let item_size = u16::from_ne_bytes(*size_buf); //получаем размер Itme
            pos += SIZE_STEP;

            let bytes = header[pos .. pos + item_size as usize].to_vec();

            let item = Item::item_from_bytes(item_size, &bytes);
            let file_name = item.file_name.clone();

            println!("{:?}", item);

            data_size += item.file_size;
            items.push(item);  
            indexes.insert(file_name, items.len() - 1);   
            
            pos += item_size as usize;               
            
        }
        println!("Data size {}", data_size);

        let tmp_file_name = format!("~{}", file_name);

        FileOper::copy_from(&tmp_file_name, file_name, header_size, data_size)?;

        Ok(Self {count_items, file_name: file_name.to_string(), 
                items, indexes, tmp_file_name,})
    }


    //извлечение файла из архива и сохранение его на диск
    pub fn take_item(&self, file_name: &str) ->  io::Result<()> {

        if let Some(index) = self.indexes.get(file_name) {
            let item = &self.items[*index];
            self.take_save(&self.tmp_file_name, &item)
        } else {
            return Err(Error::new(io::ErrorKind::NotFound, "Файл не найден."));
        }
    }

    pub fn delete_item(&mut self, file_name: &str) ->  io::Result<()> {

        if let Some(index) = self.indexes.get(file_name) {
            let item = &self.items[*index];
            self.take_save(&self.tmp_file_name, &item)?;
            self.items.remove(*index);
            self.reindexes();
            self.count_items -= 1;
            Ok(())
        }
        else {
            return Err(Error::new(io::ErrorKind::NotFound, "Файл не найден."));
        }
    }

    pub fn list_items(&self) {
        println!("\nСостав архива:");
        for item  in self.items.iter() {
            println!("{} - {}, размер {}, позиция {}", 
                item.file_name, item.mime_type, item.file_size, item.file_pos);
        }
    }
    
    
    // fn calculate_positions(&mut self, header_size: u64) {
    //     println!("\nРазмер заголовка {}", header_size);
    //     let mut pos: u64 = header_size;
    //     for item in self.items.iter_mut()  {
    //         println!("Файл {} позиция {}", item.file_name, pos);
    //         item.file_pos = pos;
    //         pos += item.file_size;
    //     }
    // }

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

    fn reindexes(&mut self) {
        self.indexes.clear();
        let mut pos = 0u64;

        for (i, item) in self.items.iter_mut().enumerate() {
            //перерасчитываем позиции смещения файлов
            item.file_pos = pos;
            pos += item.file_size;
            self.indexes.insert(item.file_name.clone(), i);
        }
    }

    // fn add_to_tmp(&self, file_name: &str, size: u64) -> io::Result<()> {

    //     Ok(())
    // }

}

