struct Point<T> {
    x: T,

    y: T,
}

enum SomeEnum<T> {
    OptionalA(T),
    OptionalB(T),
    OptionalC,
}

trait SomeCustomTrait {
    fn fun(&self, a: &str, b: &str) -> String;
}

#[allow(dead_code)]
struct MyData<T, U> {
    data_t: T,
    data_u: U,
}

impl<T, U> MyData<T, U> 
where T: std::fmt::Debug,
      U: std::fmt::Debug  {
    fn log_something(&self) {
        println!("{:?} {:?}", self.data_t, self.data_u);
    }
}

#[allow(dead_code)]
fn do_this<T>(some_var: &T) ->String 
where T: SomeCustomTrait + std::fmt::Debug {
    println!("{:?}", some_var);
    some_var.fun("first", "second")
}

#[allow(dead_code)]
fn do_this2(some_var: &dyn SomeCustomTrait) ->String {
    some_var.fun("first", "second")
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

    let ret = gen_func2(5, 10, 34.45);

    let test = MyData {
        data_t: 5.8,
        data_u: vec![1, 2, 3],
    };
    test.log_something();
}

fn gen_func<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>(param_a: T, param_b: T) -> T {
    println!("input_a has {:?}", param_a);
    param_a - param_b
}

fn gen_func2<T, E>(param_a: T, param_b: T, param_e: E) -> T 
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug, 
E: std::fmt::Debug {
    println!("input_a has {:?}", param_a);
    println!("input_e has {:?}", param_e);
    param_a - param_b
}