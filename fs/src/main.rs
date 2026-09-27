
mod menu;
use menu::{get_menu_item, get_file_name, MAIN_MENU, TASK_MENU};
mod vfs;
use vfs::Vfs;

enum MenuResult {
    Create(String),
    Open(String),
    Exit,
    NextIter,
}

fn main() {

    println!("Программа управления архивами!!\n");    
    loop {
        println!("-----------------------------------------------------------------"); 
        let oper = main_oper();
        match oper {
            MenuResult::Create(f) => {
                let mut vfs = Vfs::create(&f);
                println!("\nСоздали Vfs {}", f);
                task_oper(&mut vfs);
                
            },
            MenuResult::Open(f) => {
                if let Ok(mut vfs) =  Vfs::open(&f){
                    println!("\nОткрыли Vfs {}", f);
                    task_oper(&mut vfs);    
                } else {
                    println!("Ошибка открытия архива!");
                }
            },
            MenuResult::Exit => break,
            MenuResult::NextIter => {},
        }
    }
   
}


fn main_oper() -> MenuResult {

    let main_menu = MAIN_MENU.to_vec();
    let menu_item: usize;
 
    menu_item = get_menu_item("Выберите операцию", &main_menu);

    match menu_item {
        1 => {
            if let Some(f) = get_file_name() {
                MenuResult::Create(f)
            }
            else {
                MenuResult::NextIter
            }
        }, 
        2 => {
            if let Some(f) = get_file_name() {
                MenuResult::Open(f)
            }
            else {
                MenuResult::NextIter
            }
        },
        3 => MenuResult::Exit,
        _ => MenuResult::NextIter,
    }
    
}

fn task_oper(vfs: &mut Vfs) -> usize {

    let task_menu = TASK_MENU.to_vec();
    let mut menu_item: usize;
    loop {
        menu_item = get_menu_item("Выберите операцию", &task_menu);

        match menu_item {
            1 => {
                println!("Выбрали: показать список");
                vfs.list_items();
            },
            2 =>  {
                println!("Выбрали: добавить в архив");
                if let Some(name) = get_file_name() {
                     vfs.add(&name);
                }
            },

            3 => {
                println!("Выбрали: извлечь из архива");
                if let Some(name) = get_file_name() {
                    match vfs.take_item(&name) {
                        Ok(_) => println!("Файл: {} извлечён из архива", name),
                        Err(err) => println!("Произошла ошибка при извлечении файла: {}", err),
                    }
                }
            },
            4 =>  {
                println!("Выбрали: удалить из архива");
                // if let Some(name) = get_file_name() {
                //     vfs.add(&name);
                // }
            },
            5 => {
                println!("Выбрали: сохранить архив");
                if let Err(err) = vfs.save() {
                    println!("Ошибка при записи архива на диск: {}", err);
                } else {
                    println!("Архив успешно записан на диск.")
                }

            },
            6 => break, 
            _ => {}
        }
    }
    menu_item
}