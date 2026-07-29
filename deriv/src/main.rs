
#[allow(unused_variables)]

#[derive(Clone)]
pub struct Foo {
    x: u32,
    y: u32,
    z: Bar,
} 

use std::fmt;

impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Foo")
        .field("x", &self.x)
        .field("y", &self.y)
        .field("z", &self.z.x)
        .finish()
    }
}

#[derive(Clone)]
struct Bar {
    x: u32,
}


fn main() {
    let example_str = "Howdy";
    let example_string = String::from("Partner");

    let string_from_str = example_str.to_string();
    let string_from_str2 = "Some hardcoded string".to_string();    

    let string_from_hardcoded = String::from("Some hardcoded string");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_str;

    let combine_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "first", "second");

    let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcode literal");
    mut_string.push('m');

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;

    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(ch) => println!("Found a char {}", ch),
        None => {}
    }

    if let Some(ch) = example_str.chars().nth(2) {
        println!("Found a char {}", ch);
    }

    let string_slice_var = "Howdy";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a real string");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);

    some_string_procedure(string_var);
    //some_string_procedure(string_var);

    let some_tuple = (2, 3.4);
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("My data is {:?}", some_tuple);

}


fn some_str_procedure(param: &str) {
    println!("I'm some_str_procedure {}", param);
}

fn some_string_procedure(param: String) {
    println!("I'm some_str_procedure {}", param);
}