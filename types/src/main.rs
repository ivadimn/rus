#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//unit-структура
struct Unit;

//Кортежная структура
struct Pair(i32, f32);

//Структура с двумя полями
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}



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

fn fopt(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    Some(a? + b?)
}

fn main() {
    
    //создадим структуру при помощи сокращённой инициализации полей
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    //инициализируем Point
    let point: Point = Point { x: 10.3, y: 0.4 };

    //получаем доступ к полям структуры
    println!("Координаты точки: ({}, {})", point.x, point.y);

    let bottom_right = Point {x: 5.2, ..point };

    // bottom_right.y будет тем же самым, что и point.y 
    // так как взяли это из point
    println!("вторая точка ");



    
}
