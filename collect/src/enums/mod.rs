pub enum PlayerState {
    Standing,
    Walking,
    Running,
    Jumping,
}

pub enum Message {
    Text(String),
    Position { x: i32, y: i32},
    Color(u8, u8, u8),
    Quit,
}

   pub enum Temperature {
       Celsius(f64),
       Fahrenheit(f64),
   }

impl Temperature {
    pub fn celsius(temp: f64) -> Self {
        Self::Celsius(temp)
    }

    pub fn fahrenheit(temp: f64) -> Self {
        Self::Fahrenheit(temp)
    }

    pub fn to_celsius(&self) -> f64 {
        match self {
            Self::Celsius(c) => *c,
            Self::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
        }
    }

    pub fn to_fahrenheit(&self) -> f64 {
        match self {
            Self::Celsius(c) => c * 9.0 / 5.0 + 32.0,
            Self::Fahrenheit(f) => *f,
        }
    }
}



pub fn process_message(msg: Message) {
    match msg {
        Message::Text(text) => println!("Получено текстовое сообщени: {}", text),
        Message::Position { x, y } => println!("Полученв координаты x = {}, y = {}", x, y),
        Message::Color(r, g, b) => println!("Получен цвет: RGB({}, {}, {})", r, g, b),
        Message::Quit => println!("Получена команда завершения"),
    }
}