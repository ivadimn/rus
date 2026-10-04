use std::{fs::File, io::{self, BufReader, Read, Write}};

const MAX_BUFFER_SIZE: u64 = 65536;

pub fn get_num(prompt: &str, max_item: usize) -> usize {

    let mut input = String::new();
    let mut num: usize;
    loop {
        print!("{}:> ", prompt);
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }
        
        num = input.trim().parse().unwrap_or(0);
        if num == 0 || num > max_item { 
           println!("Неправильный пунк меню...");
           continue; 
        }
        else {
            break;
        }
    }
    num
}

pub fn get_str(prompt: &str) -> Option<String> {
    let mut input = String::new();
    
    loop {
        print!("{}:> ", prompt);
        io::stdout().flush().unwrap();    
        input.clear();

        if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }

        input = input.trim().replace("/\\", "");
        if input.len() == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }
        else {
            break;
        }
    }
    if input.to_lowercase() == "stop" {
        None
    }
    else {
        Some(input)
    }
    
}


pub fn get_few_str(prompt: &str) -> Option<Vec<String>> {
    let mut input = String::new();
        
    loop {
        print!("{}:> ", prompt);
        io::stdout().flush().unwrap();    
        input.clear();

        if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }

        if input.len() == 0 {
            println!("Попробуйте ещё раз...");
            continue;
        }
        else {
            break;
        }
    }
    let strs: Vec<String> = input.trim().split(" ").map(|s| s.to_string()) .collect();
    if strs[0].to_lowercase() == "stop" {
        None
    }
    else {
        Some(strs)
    }
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

pub fn write_buffered(src: &str, archive: &mut File) -> io::Result<()> {

    let file = File::open(src)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 65536]; // Буфер на 64 КБ
    loop {
        let readed = reader.read(&mut buffer)?;
        if readed == 0 {
            break;
        }
        let wslice = &buffer[.. readed];
        archive.write_all(&wslice)?;
    }

    Ok(())
}

pub fn write_full(src: &str, archive: &mut File) -> io::Result<()> {

    let mut file = File::open(src)?;
    let mut buffer: Vec<u8> = Vec::new();

    let _readed = file.read_to_end(&mut buffer)?;
    archive.write_all(&buffer)?;
    
    Ok(())
}

pub fn read_buffered(archive: &mut BufReader<File>, dst: &str, size: u64) -> io::Result<()> {

    let mut file = File::create(dst)?;
    let mut buffer = [0u8; 65536]; // Буфер на 64 КБ
    let mut total_read = 0usize;
    let mut up_bound: usize;
    loop {
        let readed = archive.read(&mut buffer)?;
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
        file.write_all(&wslice)?;
        if up_bound < MAX_BUFFER_SIZE as usize {
            break;
        }
    }
    Ok(())
}

pub fn read_full(archive: &mut File, dst: &str, size: u64) -> io::Result<()> {

    let mut buffer = vec![0u8; size as usize];

    archive.read_exact(&mut buffer)?;

    if buffer.len() != size as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, 
            "Прочитано неправильное количество байт."));
    }
    let mut file = File::create(dst)?;
    file.write_all(&buffer)?;

    Ok(())
}
