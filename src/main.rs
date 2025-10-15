mod db;

use iced::widget::{button, column, text};
use iced::Element;
use db::{database::connect};

#[derive(Debug, Clone)]
enum Message{
    Increment,
    Decrement
}


fn update(counter: &mut i32, message: Message){
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
    }
}


fn view(counter: &i32) -> Element<Message> {
    column![
        text(counter).size(20),
        button("Increment").on_press(Message::Increment),
        button("Decrement").on_press(Message::Decrement),
    ].spacing(10).into()

    
}

pub fn main() {
    connect();
}