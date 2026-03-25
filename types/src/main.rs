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

enum Fooo {
    Bar,
    Baz(u32, u64),
    Zoo {
        val: u64,
        flag: bool,
    }
}


fn main() {
    
    
}
