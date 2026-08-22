mod enums;
use enums::*;

// 1. Классическая структура с именованными полями
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Ассоциированная функция (конструктор)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Метод, использующий разделяемое заимствование
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Метод, требующий изменяемого заимствования
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    // Метод, потребляющий структуру
    fn destroy(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[derive(Debug, Clone)]
struct Configuration {
    host: String,
    port: u16,
    max_connections: u32,
    timeout: std::time::Duration,
}

#[derive(Default)]
struct ConfigurationBuilder {
    host: Option<String>,
    port: Option<u16>,
    max_connections: Option<u32>,
    timeout: Option<std::time::Duration>,
}

impl Configuration {
    fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::default()
    }
}

impl ConfigurationBuilder {
    fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    fn build(self) -> Result<Configuration, String> {
        Ok(Configuration {
            host: self.host.ok_or("Host is required")?,
            port: self.port.unwrap_or(8080),
            max_connections: self.max_connections.unwrap_or(100),
            timeout: self.timeout.unwrap_or(std::time::Duration::from_secs(30)),
        })
    }
}

struct NonNegativeBalance {
    value: f64,
}

impl NonNegativeBalance {
    pub fn new(value: f64) -> Result<NonNegativeBalance, &'static str> {
        if value >= 0.0 {
            Ok(NonNegativeBalance { value })
        } else {
            Err("Value is not positive")
        }
    }

    pub fn add(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount < 0.0 {
            return Err("Amount is negative");
        }
        self.value += amount;
        Ok(())
    }
     pub fn subtract(&mut self, amount: f64) -> Result<(), &'static str> {
         if amount < 0.0 {
             Err("Amount is negative")
         } else  if self.value < amount {
             Err("Not enough money")
         } else {
             self.value -= amount;
             Ok(())
         }
     }
    pub fn get_value(&self) -> f64 {
        self.value
    }

}



fn main() {
    let mut numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Безопасный доступ к элементам
    let first = numbers[0];  // Получаем первый элемент
    println!("Первый элемент: {}", first);

    // Доступ с проверкой границ
    if let Some(&element) = numbers.get(1) {
        println!("Второй элемент: {}", element);
    }

    // Это вызовет панику во время выполнения
    // let invalid = numbers[10];  // ❌ Паника: индекс вне границ

    // Безопасная проверка существования элемента
    match numbers.get(10) {
        Some(value) => println!("Элемент найден: {}", value),
        None => println!("Элемент не существует"),
    }

    // Изменение нескольких элементов в цикле
    for i in 0..numbers.len() {
        numbers[i] *= 2;
    }

    println!("Измененный массив: {:?}", numbers);

    let slice1: &[i32] = &numbers[1..4];
    let slice2: &[i32] = &numbers[..3];
    let slice3: &[i32] = &numbers[7..];
    let slice4: &[i32] = &numbers[..];

    print_slice(&slice1);
    print_slice(&slice2);
    print_slice(&slice3);
    print_slice(&slice4);

    // Двумерный массив (матрица 3x3)
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    //доступ к элемннтам
    println!("Элемент matrix[1][1]: {}", matrix[1][1]);
    // Обход всех элементов
    for row in &matrix {
        for &element in row {
            print!("{} ", element);
        }
        println!();
    }
    // Пример 1: Расчет средней температуры
    let temperatures = [22.5, 23.1, 24.0, 22.9, 23.5, 22.8, 23.2];
    println!("Средняя температура: {:.1}°C",
             calculate_average(&temperatures));

    // Пример 2: Поиск максимального значения
    let scores = [85, 92, 78, 95, 88, 90];
    if let Some(max_score) = find_max(&scores) {
        println!("Максимальный балл: {}", max_score);
    }

    // Пример 3: Обработка фиксированного набора данных
    let rgb_color = [255, 128, 0]; // Оранжевый цвет в RGB
    println!("Цвет RGB: {:?}", rgb_color);

    // Создание экземпляра структуры
    let mut user1 = User {
        email: String::from("user@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    // Изменение поля (требуется mut)
    user1.email = String::from("newemail@example.com");

    // Создание структуры из другой структуры
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1  // синтаксис обновления структуры
    };

    println!("{}", user1.email);

    let rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());
    let (w, h) = rect.destroy();
    println!("Area: {}", w * h);

    let builder = Configuration::builder();
    let conf = builder.host("localhost".to_string())
        .port(8080)
        .build().unwrap();
    println!("Configuration: {:?}", conf);

    let nv = NonNegativeBalance::new(100.0);
    match nv {
        Ok(nv) => {println!("Balance {}", nv.get_value())},
        Err(msg) => {println!("Error: {}", msg)}
    }

    let state = PlayerState::Running;
    match state {
        PlayerState::Standing => println!("Standing"),
        PlayerState::Walking => println!("Walking"),
        PlayerState::Running => println!("Running"),
        PlayerState::Jumping => println!("jumping"),
    }

    let messages = vec![
        Message::Text("Hello".to_string()),
        Message::Position { x: 20, y: 10 },
        Message::Color(100, 56, 78),
        Message::Quit,
    ];

    for message in messages {
        process_message(message);
    }

    let temp1 = Temperature::celsius(25.0);
    let temp2 = Temperature::fahrenheit(98.6);

    println!("{}C {}F", temp1.to_celsius(), temp1.to_fahrenheit());
    println!("{}F {}C", temp2.to_fahrenheit(), temp2.to_celsius());

    let numbers = vec![(10.0, 2.0), (8.0, 0.0), (24.0, 6.0)];

    for (num, den) in numbers {
        match divide(num, den) {
            Some(result) => println!("{} / {} = {}", num, den, result),
            None => println!("{} / {} деленин на 0", num, den),
        }
    }

    if let Some(result) = divide(10.0, 2.0) {
        println!("Результат: {}", result);
    }
    let some_number = Some(10);
    let absent_number: Option<i32> = None;

    print_number(some_number);
    print_number(absent_number);

    let some_value: Option<i32> = Some(5);
    let unwrapped_value = some_value.unwrap_or(10);
    println!("Unwrapped value: {}", unwrapped_value); // Вывод: 5

    let none_value: Option<i32> = None;
    let default_value = none_value.unwrap_or(10);
    println!("Default value: {}", default_value); // Вывод: 10

    let some_value: Option<i32> = Some(5);
    let mapped_value = some_value.map(|x| x * 2);
    println!("Mapped value: {:?}", mapped_value); // Вывод: Some(10)

    let none_value: Option<i32> = None;
    let default_value = none_value.map(|x| x * 2);
    println!("Mapped value: {:?}", default_value); // Вывод: None


}

fn print_number(opt_number: Option<i32>) {
    match opt_number {
        Some(num) => println!("Number: {}", num),
        None => println!("No number provided"),
    }
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn print_slice(slice: &[i32]) {
    println!("Slice: {:?}", slice);
}

fn calculate_average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().max().copied()
}