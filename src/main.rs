mod db;
mod ui;
mod app;
use app::update;
use app::view;



pub fn main() -> iced::Result{
    db::database::setup();
    iced::run("Habit tracker RS", update, view)
}