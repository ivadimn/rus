use std::sync::Arc;
use std::convert::From;
use std::convert::TryFrom;


fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn apply_operation(x: i32, y: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(x, y)
}

struct Kilometers(f64);
struct Miles(f64);

impl From<Miles> for Kilometers {
    fn from(value: Miles) -> Self {
        Kilometers(value.0 * 1.609344)
    }
}
#[derive(Debug)]
struct PositiveNumber(i32);

impl TryFrom<i32> for PositiveNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value))
        } else {
            Err(String::from("Число должно быть положительным"))
        }
    }
}


fn main() {
    let data = Arc::new(42);
    let clone1 = Arc::clone(&data);
    let clone2 = Arc::clone(&data);

    println!("Reference count: {}", Arc::strong_count(&data));

    let sum: fn(i32, i32) -> i32 = add;
    println!("Summa: {}", sum(4, 8));
    println!("Apply operation: {}", apply_operation(3, 8, add));

    let large_value: i32 = 1000;
    let small_value: i8 = large_value as i8; // Потенциально потеря данных

    println!("Преобразование типа: {:b}", small_value);

    let miles = Miles(5.0);
    let kilometers: Kilometers = miles.into();
    println!("5 миль это {} километров", kilometers.0);

    let positive = PositiveNumber::try_from(42);
    let negative = PositiveNumber::try_from(-42);

    println!("Положительное: {:?}", positive);
    println!("Отрицательное: {:?}", negative);

}
