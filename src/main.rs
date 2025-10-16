mod db;
mod app;
mod services;
use app::update;
use app::view;




pub fn main() -> iced::Result{
    if std::env::args().any(|a| a == "--reset-db") {
        db::database::setup();
    }

    iced::application("Habit tracker", update, view)
        .theme(|_s| iced::Theme::GruvboxDark)
        .run()
}