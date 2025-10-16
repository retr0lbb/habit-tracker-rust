mod db;
mod app;
mod services;
use app::update;
use app::view;




pub fn main() -> iced::Result{
    db::database::setup();
    iced::application("Habit tracker", update, view).theme(|_s| iced::Theme::KanagawaDragon).run()
}