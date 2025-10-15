use iced::widget::{button, column, text};
use iced::Element;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    Close,
}

pub fn update(counter: &mut i32, message: Message){
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
        Message::Close => {
            std::process::exit(0);
        }
    }
}


pub fn view(counter: &i32) -> Element<Message> {
    column![
        text(counter).size(20),
        button("Increment").on_press(Message::Increment),
        button("Decrement").on_press(Message::Decrement),
        button("Close APP").on_press(Message::Close)
    ].spacing(10).into()

}