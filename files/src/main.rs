//use regex::Regex;
//use std::sync::{Arc, Mutex};
//use std::thread;
use std::fmt::{self, write};
use std::mem;

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

fn apply<F>(f: F) where 
    F: FnOnce() {

    f();
}

fn apply_to_3<F>(f: F) -> i32 where 
    F: Fn(i32) -> i32 {
        f(3)
    }

#[allow(unreachable_code)]
fn main() {
    let greeting = "привет";

    let mut farewell = "пока".to_owned();

    // Захват двух переменных: `greeting` по ссылке и
    // `farewell` по значению.
    let diary = || {
        // `greeting` захватывается по ссылке: требует `Fn`.
        println!("Я сказал {}.", greeting);

        // Изменяемость требует от `farewell` быть захваченным
        // по изменяемой ссылке. Сейчас требуется `FnMut`.
        farewell.push_str("!!!");
        println!("Потом я закричал {}.", farewell);
        println!("Теперь я могу поспать. zzzzz");

        // Ручной вызов удаления требуется от `farewell`
        // быть захваченным по значению. Теперь требуется `FnOnce`.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| x * 2;

    println!("Удвоенное 3: {}", apply_to_3(double));


}
