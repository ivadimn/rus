mod math;
mod practice;

use crate::math::basic;
use crate::practice::generate_task;

fn main() {

    let (a, b) = generate_task();
    
    println!("Тренировочный пример {a} + {b} = {}", basic::add(a, b));
    println!("12 - 4 = {}", basic::subtract(12, 4));
    println!("6 * 7 = {}", basic::multiply(6, 7));
    println!("20 / 5 = {}", basic::divide(20.0, 5.0));
}
