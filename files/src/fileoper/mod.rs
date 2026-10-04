use std::io::{self, BufReader, Error, ErrorKind, Read, Seek, Write, BufWriter};
use std::fs::{File, Metadata, metadata, OpenOptions};

const MAX_BUFFER_SIZE: u64 = 65536;

pub struct FileOper;

impl FileOper {
        
    //будем использовать для записи  и дозаписи во временный файли 
    //файлы источники читаются целиком и дописываются в конец архива
    pub fn copy_to(dst: &str, src: &str, count: u64) -> io::Result<()> {

                      
        let mut dst_file = OpenOptions::new()
                                        .create(true)
                                        .append(true)
                                        .open(dst)?;
        let mut src_file = File::open(src)?;
        if count > MAX_BUFFER_SIZE {
            let mut reader  = BufReader::new(src_file);
            let mut writer = BufWriter::new(dst_file);
            Self::bfcopy(&mut writer, &mut reader, count)    
        }
        else {
            Self::ffcopy(&mut dst_file, &mut src_file, count)
        }
    }

    //будем использовать для извлечения из архива и записи в файл
    //или при записи во времннный файл при открытии архива
    pub fn copy_from(dst: &str, src: &str, position: u64, count: u64) -> io::Result<()> {

        let file_attr: Metadata = metadata(src)?;
        if position + count > file_attr.len() {
            return Err(Error::new(ErrorKind::InvalidData, 
                "Требуемое количество байт превышает размер файла."));
        }
                
        
        let mut dst_file = File::create(dst)?;
        let mut src_file = File::open(src)?;
        
        if count > MAX_BUFFER_SIZE {
            let mut reader  = BufReader::new(src_file);
            let mut writer = BufWriter::new(dst_file);
            reader.seek(io::SeekFrom::Start(position))?;
            Self::bfcopy(&mut writer, &mut reader, count)    
        }
        else {
            src_file.seek(io::SeekFrom::Start(position))?;
            Self::ffcopy(&mut dst_file, &mut src_file, count)
        }
    }

    fn bfcopy(dst: &mut BufWriter<File>, src: &mut BufReader<File>, size: u64) -> io::Result<()> {

        let mut buffer = [0u8; MAX_BUFFER_SIZE as usize];
        let mut total_read = 0usize;
        let mut up_bound: usize;
        loop {
            let readed = src.read(&mut buffer)?;
            total_read += readed;
            if readed == 0  {
                break;
            }
            up_bound = if total_read > size as usize {
               MAX_BUFFER_SIZE as usize - (total_read - size as usize)
            }
            else {
                readed
            };
            let wslice = &buffer[.. up_bound];
            dst.write(wslice)?;
            if up_bound < MAX_BUFFER_SIZE as usize {
                break;
            }
        }
        let _= dst.flush();
        Ok(())
    }

    fn ffcopy(dst: &mut File, src: &mut File, size: u64) -> io::Result<()> {

        let mut data = vec![0u8; size as usize];
        src.read_exact(&mut data)?;
        dst.write(&data)?;
        Ok(())
    } 

}