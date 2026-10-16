use iced::widget::{Column, button, column, text};


#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
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

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
            ]
    } 
}

fn main() -> iced::Result {
    iced::run("Счётчик", Counter::update, Counter::view)

}
