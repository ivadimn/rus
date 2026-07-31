struct Point<T> {
    x: T,
    y: T,
}

enum SomeEnum<T> {
    OptionalA(T),
    OptionalB(T),
    OptionalC,
}


fn main() {

    let point_a = Point {x: 20, y: -10};
    println!("Point A {} {}", point_a.x, point_a.y);

    let point_b = Point {x: 20.7, y: -10.4};
    println!("Point A {} {}", point_b.x, point_b.y);

    let some_data = SomeEnum::OptionalA(-34.56);

    match some_data {
        SomeEnum::OptionalA(a) => println!("OptionalA contains {}", a),
        SomeEnum::OptionalB(b) => println!("OptionalB contains {}", b),
        SomeEnum::OptionalC =>  println!("Boring optional C"),
    }
    let some_data2 = SomeEnum::OptionalB('C');
    let some_data3 = SomeEnum::OptionalA(vec![1, 2, 3]);

    let ret = gen_func(5, 10);
}

fn gen_func<T: std::ops::Add<Output=T>>(param_a: T, param_b: T) -> T {
    param_a + param_b
}
