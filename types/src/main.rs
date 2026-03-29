mod foo {
    #[derive(Debug, Default)]
    pub struct Foo {
        pub val: u32,
        pub bar: (u64, u128, bool),
        pub flag: bool,
    }
}

use std::backtrace;

pub use foo::Foo;

fn f() -> Foo {
    Foo {
        flag: true,
        ..Foo::default()
    }
}

fn f1() -> Foo {
    Foo {
        flag: true,
        ..{
            let mut x = Foo::default();
            x.bar.1 = 42;
            x
        } 
    }
}
struct InitToken;
fn init() -> InitToken {
    //init code
    InitToken
}

fn do_stuff(token: InitToken) {
    // stuff
}

#[derive(Debug)]
enum Fooo {
    Bar,
    Baz(u32, u64),
    Zoo {
        val: u64,
        flag: bool,
    },
    Moo(Moo),
}

#[derive(Debug)]
#[repr(C)]
pub struct Moo {
    b: u32,
    a: u128,
    c: u32,
}

enum Bar {
    A {a: u32}, 
    B {a: u64, c: (u32, u64)},
}

#[repr(align(4096))]
struct Page([u8; 4096]);

fn inc_val(item: &mut i32) {
    *item += 1;
}

fn main() {
    
    let mut x = Page([0; 4096]);
    x.0[0] = 42;

    println!("{}", x.0[0]);

    let mut y: i32 = 0;
    inc_val(&mut y);
    println!("{}", y);

    let arr = [1, 2, 3, 4, 5, 6];
    for item in arr.iter() {
        //inc_val(mut item);
        println!("{}", item);
    } 
    
}
