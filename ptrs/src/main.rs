use std::rc::Rc;

fn main() {
    let data = Rc::new(42);
    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);

    println!("Reference count: {}", Rc::strong_count(&data));
}
