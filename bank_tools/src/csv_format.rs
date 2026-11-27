use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use crate::model::Transaction;
use crate::error::ToolsError;


pub fn read_csv(file_name: &str) -> Result<Vec<Transaction>, ToolsError> {

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    // let v: Vec<_> = reader.lines()
    //     .map(|line| line?)
    //     .
 
    // let iter = reader.lines();
    
    // let Some(line) = iter.next() else {
    //     return Err(ToolsError::FileFormatError(file_name.into()));
    // };
    
    // while let Some(line) = iter.next() {
    //     let parts = line.;
    // }
    
    for (index, line)  in reader.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let line = line?;
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 4 {
            return Err(ToolsError::FileFormatError(file_name.into()));
        }
        let date_part:Vec<&str> = parts[0].split('-').collect();
        let year = date_part[0].parse::<u16>();
    }

    Ok(vec![])
} 