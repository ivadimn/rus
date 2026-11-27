use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum DataError {
    ParseDateError(String, ParseIntError),
    ParseAmountError(String, ParseIntError),
    InvalidOperation(String, String),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::ParseDateError(fname, err) => 
                write!(f, "Ошибка при обработке даты: {} в файле {}", err, fname),
            DataError::ParseAmountError(fname, err) => 
                write!(f, "Ошибка при обработке суммы: {} в файле {}", err, fname),
            DataError::InvalidOperation(fname, op) => 
                write!(f, "Не правильная операция: {} в файле {}", op, fname),
        }
    }
}

#[derive(Debug)]
pub enum ToolsError {
    FileOpError(io::Error),
    FileFormatError(String),
    FileDataError(String, DataError),
}

impl std::fmt::Display for ToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           ToolsError::FileFormatError(fname) => write!(f, "Ошибка формата файла: {}", fname),
           ToolsError::FileOpError(err) => write!(f, "Ошибка: {}", err),
           ToolsError::FileDataError(fname, describe) => 
                    write!(f, "Ошибка при обработке файла {}: {}", fname, describe),
        }
    }
}

impl std::error::Error for ToolsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
       match self {
           Self::FileOpError(s) => None,
           Self::FileFormatError(s) => None,
           Self::FileDataError(fname, data_error ) => Some()
       } 
    }
}

impl From<io::Error> for ToolsError {
    fn from(value: io::Error) -> Self {
        ToolsError::FileOpError(value)
    }
}
