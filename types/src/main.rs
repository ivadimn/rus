use std::fmt;

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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x = {}, y = {}", self.x, self.y)
    }
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

fn double_int32(number: i32) -> i32 {
    number * 2
}

fn double_int64(number: i32) -> i64 {
    number as i64 * 2
}

fn int_plus_float_to_float(n1: i32, n2: f32) -> f64 {
    n1 as f64 + n2 as f64
}

fn int_plus_float_to_int(n1: i32, n2: f32) -> i64 {
    n1 as i64 + n2 as i64
}

fn tuple_sum(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn array_sum(a: [i32; 3]) -> i32 {
    let mut sum = 0;
    for n in a {
        sum += n;
    }
    sum
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
    println!("Координаты точки: ({})", point);

    let bottom_right = Point {x: 5.2, ..point };

    // bottom_right.y будет тем же самым, что и point.y 
    // так как взяли это из point
    println!("вторая точка {}", bottom_right);

    let r1 = double_int32(56);
    println!("{r1}");
    let r2 = double_int64(526);
    println!("{r2}");
    let r3 = int_plus_float_to_float(31, 0.004);
    println!("{:.3}", r3);
    let r4 = int_plus_float_to_int(31, 30.004);
    println!("{}", r4);
    let r5 = tuple_sum((23, 78));
    println!("{}", r5);
    let r6 = array_sum([10, 20, 30]);
    println!("{}", r6);

}
