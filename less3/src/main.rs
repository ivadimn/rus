fn main() {
    let y = 2023;
    let y = y + 1;
    {
        let y = y - 24;
        println!("{y}");
    }
    println!("{y}");
}
