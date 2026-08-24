use std::{rc::Rc, sync::Arc};

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn apply_operation(x: i32, y: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(x, y)
}

fn main() {
    let data = Arc::new(42);
    let clone1 = Arc::clone(&data);
    let clone2 = Arc::clone(&data);

    println!("Reference count: {}", Arc::strong_count(&data));

    let sum: fn(i32, i32) -> i32 = add;
    println!("Summa: {}", sum(4, 8));
    println!("Apply operation: {}", apply_operation(3, 8, add));
}
