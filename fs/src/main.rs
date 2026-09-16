
mod menu;
use menu::{get_menu_item, get_file_name, MAIN_MENU};
mod vfs;
use vfs::Vfs;

fn main() {
    
    let main_menu = MAIN_MENU.to_vec();

    let mut vfs = Vfs::default();
    loop {
        let menu_item = get_menu_item("Выберите операцию", &main_menu);

        match menu_item {
            1 =>  {
                if let Some(name) = get_file_name() {
                    vfs = Vfs::create(&name).unwrap();
                    vfs.save().unwrap();
                    println!("Create"); 
                }
            },

            2 => println!("Open"),
            3 => println!("Delete"),
            4 => {
                vfs.save();
                println!("save");
            }
            5 => break,
            _ => {}
        }
    }
    
}
