struct Foo {
    val: u32,
    flag: bool,
}

fn f() -> Foo {
    let val = 42;
    Foo {
        val, 
        flag: true,
    }
}
pub enum FooE {
    Bar,
    Baz(u32, u64),
    Zoo {
        val: u32,
        flag:bool,
    }
}

fn main() {
    
    let foo = f();
    println!("{}, {}", foo.val, foo.flag);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("{five_hundred}");

}
