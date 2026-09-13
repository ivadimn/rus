
mod menu;
use menu::{get_menu_item, MAIN_MENU};
mod vfs;
use vfs::Vfs;

fn main() {
    let mut vfs: Vfs;
    let main_menu = MAIN_MENU.to_vec();

    let mut vfs: Vfs;
    loop {
        let menu_item = get_menu_item("Выберите операцию", &main_menu);

        match menu_item {
            1 =>  {
                vfs = Vfs::create("test.data").unwrap();
                println!("Create"); },
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
