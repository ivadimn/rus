
mod menu;
use menu::{get_menu_item, get_file_name, MAIN_MENU, TASK_MENU};
mod vfs;
use vfs::Vfs;

fn main() {

    let mut is_main = true;
    let mut vfs = Vfs::default();

    loop {
        println!("is_main: {}", is_main);
        if is_main {
            let res = main_oper(&mut vfs);
            if res == 5 {
                break;
            }
            else {
                is_main = false
            }

        }
        else {
            let res = task_oper(&mut vfs);
            if res == 5 {
                is_main = true;
            }
        }
    }
   
}


fn main_oper(vfs: &mut Vfs) -> usize {

    let main_menu = MAIN_MENU.to_vec();
    let mut menu_item: usize;
 
    menu_item = get_menu_item("Выберите операцию", &main_menu);

    match menu_item {
        1 =>  {
            if let Some(name) = get_file_name() {
                *vfs = Vfs::create(&name).unwrap();
                vfs.save().unwrap();
                println!("Create"); 
            }
        },

        2 => println!("Open"),
        3 => println!("Delete"),
        4 => {
            vfs.save();
            println!("save");
        },
        _ => {}
    }
    
    menu_item
}

fn task_oper(vfs: &mut Vfs) -> usize {

    let task_menu = TASK_MENU.to_vec();
    let mut menu_item: usize;
    loop {
        menu_item = get_menu_item("Выберите операцию", &task_menu);

        match menu_item {
            1 => {
                vfs.show_list();
            },
            2 =>  {
                if let Some(name) = get_file_name() {
                    vfs.add(&name).unwrap();
                }
            },

            3 => println!("Open"),
            4 => println!("Delete"),
            5 => break,
            _ => {}
        }
    }
    menu_item
}