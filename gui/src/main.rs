
struct Counter {
    value: i64,
}

enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message)  {
        match message {
            Message::Increment => {
                self.value += 1;
            },
            Message::Decrement => {
                self.value -= 1;
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
