mod foo {
    #[derive(Debug, Default)]
    pub struct Foo {
        pub val: u32,
        pub bar: (u64, u128, bool),
        pub flag: bool,
    }
}

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

fn main() {
    
    let t = init();
    do_stuff(t);
    
    let x = f1();
    let Foo{ flag: flag2, ..} = f();
    println!("Foo: {x:?}");
    println!("Flag: {flag2}");
}
