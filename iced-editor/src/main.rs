use iced::widget::{column, container, horizontal_space, row, text, text_editor};
use iced::{Application, Command, Element, Length, Settings, Theme, executor}; //подключаем песочницу

use std::{io, path};
use std::path::Path;
use std::sync::Arc;

//структура для состояния приложения
//здесь будет содержаться всё состояние приложения
struct Editor {
    content: text_editor::Content, //содержимое текстового редактора
}

// перечисление определяющие виды сообщений
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Application for Editor {
    type Message = Message; //тип сообщения который будет обрабатываться
    type Theme = Theme; // для темы приложения
    type Executor = executor::Default; //исполнитедль асинхронных операций
    type Flags = ();
    //возвращает асинхронную команду с сообщением
    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: text_editor::Content::with(include_str!("main.rs")),
            },
            Command::none(), //возвращаем пустую команду
        )
    }

    fn title(&self) -> String {
        String::from("A cool editor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content).on_edit(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{} : {}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), position];

        container(column![input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path).await
        .map(Arc::new)
        .map_err(|error| error.kind())
}

fn main() -> iced::Result {
    Editor::run(Settings::default())
}
