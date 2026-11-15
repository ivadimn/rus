#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]


fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}

fn main() {
    let my_string = String::from("Hello world");
    let word = get_first_word(&my_string);
    //drop(my_string);
    println!("First word: {}", word);
}