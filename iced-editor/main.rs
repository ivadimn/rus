use iced::{Sandbox, Element, Settings}; //подключаем песочницу
use iced::widget::{text_editor, container};

//структура для состояния приложения
//здесь будет содержаться всё состояние приложения

struct Editor {
    content: text_editor::Content, //содержимое текстового редактора
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;
    
    fn new() -> Self {
       Self {
        content: text_editor::Content::new(),
       }   
    }

    fn title(&self) -> String {
        String::from("A cool editor")
    }
    
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .on_edit(Message::Edit);
        container(input).padding(10).into()
    }

}

fn main() -> iced::Result{
    Editor::run(Settings::default())
}
