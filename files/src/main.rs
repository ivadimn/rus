//use regex::Regex;
//use std::sync::{Arc, Mutex};
//use std::thread;
use std::fmt::{self, write};

fn longest_common_prefix<'a>(x: &'a str, y: &'a str) -> &'a str{
    let  min_length = std::cmp::min(x.len(), y.len());
    let bytes_x = x.as_bytes();
    let bytes_y = y.as_bytes();
    for i in 0 .. min_length {
        if bytes_x[i] != bytes_y[i] {
            return &x[..i];
        }
    }
    &x[.. min_length]
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transponse(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}


#[allow(unreachable_code)]
fn main() {
    fn function(i: i32) -> i32 {i + 1}

    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = |i  | i + 1;

    let i = 1;
    println!("Функция: {}", function(i));
    println!("Замыкание с указанием типа: {}", closure_annotated(i));
    println!("Замыкание с выводом типа: {}", closure_inferred(i));

    let one = || 1;
    println!("Замыкание возвращающее один {}", one());

}
