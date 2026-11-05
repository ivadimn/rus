#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug)]
struct LoggerConfig {
    level: String,
    file_path: String,
}

#[derive(Debug)]
enum LoggerError {
    ReadConfig(std::io::Error),
    OpenLog(std::io::Error),
    MissingField(&'static str),
    InvalidLevel(String),
    EmptyPath,
}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerError::ReadConfig(_) => write!(f, "не удалось прочитать конфиг"),
            LoggerError::OpenLog(_) => write!(f, "не удалось открыть лог-файл"),
            LoggerError::MissingField(field) => write!(f, "отсутствует обязательное поле: {field}"),
            LoggerError::InvalidLevel(level) => write!(f, "неверный уровень логирования: {level}"),
            LoggerError::EmptyPath => write!(f, "путь к лог-файлу пуст"),
        }
    }
}

impl Error for LoggerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoggerError::ReadConfig(error) => Some(error),
            LoggerError::OpenLog(error) => Some(error),
            LoggerError::MissingField(_) => None,
            LoggerError::InvalidLevel(_) => None,
            LoggerError::EmptyPath => None,
        }
    }
}

fn find_value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .find_map(|line| {
            let (left, right) = line.split_once('=')?;
            (left.trim() == key).then_some(right.trim())
        })
}

fn validate_level(level: &str) -> Result<String, LoggerError> {
    match level {
        "trace" | "debug" | "info" | "warn" | "error" => Ok(level.to_string()),
        _ => Err(LoggerError::InvalidLevel(level.to_string())),
    }
}

fn validate_path(path: &str) -> Result<String, LoggerError> {
    let trimmed = path.trim();

    if trimmed.is_empty() {
        Err(LoggerError::EmptyPath)
    } else {
        Ok(trimmed.to_string())
    }
}

fn parse_config(text: &str) -> Result<LoggerConfig, LoggerError> {
    let level = find_value(text, "level")
        .ok_or_else(|| LoggerError::MissingField("level"))
        .and_then(validate_level)?;

    let file_path = find_value(text, "file")
        .ok_or_else(|| LoggerError::MissingField("file"))
        .and_then(validate_path)?;

    Ok(LoggerConfig { level, file_path })
}

fn init_logger(path: &Path) -> Result<LoggerConfig, LoggerError> {
    let text = fs::read_to_string(path).map_err(LoggerError::ReadConfig)?;
    let config = parse_config(&text)?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.file_path)
        .map_err(LoggerError::OpenLog)?;

    Ok(config)
}

fn main() -> Result<(), Box<dyn Error>> {
    let base = std::env::temp_dir().join("rust-course-logger-demo");
    fs::create_dir_all(&base)?;

    let ok_log_path = base.join("app.log");
    let ok_config_path = base.join("logger.conf");
    let missing_config_path = base.join("missing.conf");
    let bad_config_path = base.join("bad-logger.conf");
    let bad_log_path = base.join("missing-dir").join("app.log");

    fs::write(
        &ok_config_path,
        format!("level=info\nfile={}\n", ok_log_path.display()),
    )?;

    fs::write(
        &bad_config_path,
        format!("level=debug\nfile={}\n", bad_log_path.display()),
    )?;

    match init_logger(&ok_config_path) {
        Ok(config) => println!("logger ready: {} -> {}", config.level, config.file_path),
        Err(error) => println!("unexpected error: {error}"),
    }

    match init_logger(&missing_config_path) {
        Err(LoggerError::ReadConfig(_)) => println!("missing config handled"),
        Ok(_) => println!("unexpected success"),
        Err(error) => println!("unexpected error: {error}"),
    }

    match init_logger(&bad_config_path) {
        Err(LoggerError::OpenLog(_)) => println!("write error handled"),
        Ok(_) => println!("unexpected success"),
        Err(error) => println!("unexpected error: {error}"),
    }

    Ok(())
}