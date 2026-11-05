use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum LoadError {
    UserNotFound,
    KeyNotFound,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::KeyNotFound => write!(f, "ключ не найден"),
            LoadError::UserNotFound => write!(f, "пользователь не найден")
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug)]
pub enum LoginError {
    InvalidEmail(String),
    InvalidPassword,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginError::InvalidEmail(email) => 
                write!(f, "не вернный email {}", email),
            LoginError::InvalidPassword => 
                write!(f, "не правильный пароль")
        }
    }
}

impl Error for LoginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug)]
pub enum GetDataError {
    Login(LoginError),
    Load(LoadError),
}

impl fmt::Display for GetDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetDataError::Load(_) => write!(f, "ошибка загрузки"),
            GetDataError::Login(_) => write!(f, "ошибка входа"),
        }
    }
    
}

impl Error for GetDataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GetDataError::Load(lerr) => Some(lerr),
            GetDataError::Login(lerr) => Some(lerr),
        }
    }
}

impl From<LoadError> for GetDataError {
    fn from(value: LoadError) -> Self {
        match value {
            LoadError::KeyNotFound => 
                GetDataError::Load(LoadError::KeyNotFound),
            LoadError::UserNotFound => 
                GetDataError::Load(LoadError::UserNotFound),
        }
    }
}

impl From<LoginError> for GetDataError {
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::InvalidEmail(email) => 
                GetDataError::Login(LoginError::InvalidEmail(email)),
            LoginError::InvalidPassword => 
                GetDataError::Login(LoginError::InvalidPassword),
        }
    }
}

pub fn process_error(e: impl Error) {
    println!("Message for user: {}", e);
    eprintln!("Debug: {:?}", e);

    if let Some(source) = e.source() {
        eprintln!("Caused by: {}", source);
    }
}