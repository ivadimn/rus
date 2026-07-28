
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

    let a = Foo {
        x: 1, y: 2, z: Bar {x: 3},
    };
    println!("{:?}", a);
}
