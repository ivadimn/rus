use fs::{get_num, get_str};

const MAIN_MENU_COUNT: usize = 5;

pub static MAIN_MENU: [&str; MAIN_MENU_COUNT] = [
    "1. Создать и открыть новую VFS.",
    "2. Открыть VFS.",
    "3. Удалить VFS.",
    "4. Сохранить VFS",
    "4. Выход."
];

pub static TASK_MENU: [&str; 4] = [
    "1. Добавить файл в FS.",
    "2. Извлечь файл из FS.",
    "3. Удалить файл из FS.",
    "4. В главное меню."
];

pub fn get_menu_item(prompt: &str, menu: &Vec<&str>) -> usize {
    
    for menu_item in menu {
        println!("{}", menu_item);
    }
    get_num(prompt, MAIN_MENU_COUNT)
}

pub fn get_file_name() -> Option<String> {
    get_str("Введите имя файла (или <stop> для отмены)")
}