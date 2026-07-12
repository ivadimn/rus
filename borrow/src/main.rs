

fn get_mem(t : &mut (i32, i32), flag: bool) -> &mut i32 {
    let r : &mut i32;
    if flag  {
        r = &mut t.0;
    }
    else { 
        r = &mut t.1;
    }
    r
}

fn main() {

    let x = get_mem(&mut (27, 28), true);

    println!("Hello, world!");
}
