#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

//use std::error::Error;

mod handle_error;
use handle_error::{GetDataError, LoadError, LoginError, process_error};

pub fn get_data(email: &str, pass: &str, key: &str) -> Result<String, GetDataError> {
    let user_id = login(email, pass)?;
    let data = download(user_id, key)?;
    Ok(data)
}

pub fn login(email: &str, pass: &str) -> Result<u32, LoginError> {
    if email != "a@b.com" {
        return Err(LoginError::InvalidEmail(email.into()));
    }

    if pass != "123" {
        return Err(LoginError::InvalidPassword);
    }
    Ok(42)
}

pub fn download(user_id: u32, key: &str) -> Result<String, LoadError> {
    if user_id != 42 {
        return Err(LoadError::UserNotFound);
    }

    if key != "321" {
        return Err(LoadError::KeyNotFound);
    }

    Ok(String::from("data"))
}

fn main() {
    let cases = "#otion_1=123\n#otion_2=345\n#level=debug\n#path=/tmp/debug.log";
    
    // cases.lines()
    //     .map(str::trim)
    //     .filter(|s| s.starts_with("#"))
    //     
      
    
    
}
