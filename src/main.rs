mod db;

use iced::widget::{button, column, text};
use iced::Element;


#[derive(Debug, Clone)]
enum Message{
    Increment,
    Decrement,
    Close
}


fn update(counter: &mut i32, message: Message){
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
        Message::Close => {
            std::process::exit(0);
        }
    }
}


fn view(counter: &i32) -> Element<Message> {
    column![
        text(counter).size(20),
        button("Increment").on_press(Message::Increment),
        button("Decrement").on_press(Message::Decrement),
        button("Close APP").on_press(Message::Close)
    ].spacing(10).into()

}

pub fn main() -> iced::Result{
    db::database::setup();
    iced::run("Habit tracker RS", update, view)
}