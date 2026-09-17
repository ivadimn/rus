use std::io::{self, Write};



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