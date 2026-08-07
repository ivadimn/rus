//use regex::Regex;

fn main() {
    let mut buffer: [u8; 5] = [0; 5];
    let data = b"Overflowing Content";
    buffer[.. data.len()].copy_from_slice(data);
    println!("Overflowing buffer {:?}", buffer);
    
}
