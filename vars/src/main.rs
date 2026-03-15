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

fn main() {
    
    let foo = f();
    println!("{}, {}", foo.val, foo.flag);

}
