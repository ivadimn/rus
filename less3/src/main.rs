fn main() {
    let y = 2023;
    let y = y + 1;
    {
        let y = y - 24;
        println!("{y}");
    }
    println!("{y}");
    println!("{}", f64::EPSILON);
    println!("{}", 12.2 % 5.5);
    

    let l = "абвгдж".len();
    println!("{}", l);
    let c = "абвгдж".chars().count();
    println!("{}", c);
    let s = &("абвгдж")[4..];
    println!("{}", s);
    let f = "абвгдж".starts_with("абв");
    println!("{}", f);
    let f = "абвгдж".ends_with("гдж");
    println!("{}", f);
    let _o = "абвгдж".find("вг");
    //println!("{}", Some(o));
    let s = "абвгдж".replace("абв", "АБВ");
    println!("{}", s);
    //let s = "абвгдж".split(" ");
    let s = "  абвгдж   ".trim();
    println!("{}", s);
}
