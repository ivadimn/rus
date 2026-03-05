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
}
