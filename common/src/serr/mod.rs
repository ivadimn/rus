use std::error::Error;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            MyError::General(s) => write!(f, "General error: {}", s),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(s) => None,
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

