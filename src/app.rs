use iced::widget::{button, column, text_input};
use iced::{Element};
use crate::services::createHabit::{self, listHabits};

#[derive(Debug, Clone)]
pub struct Habit{
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub weekly_frequency: u8,
}

impl Default for Habit {
    fn default() -> Self {
        Habit {
            id: 0,
            name: String::new(),
            desc: String::new(),
            weekly_frequency: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Close,
    NameInputChange(String),
    DescInputChange(String),
    FrequencyInputChange(String),
    SubmitHabitCreation(),
    GetHabits
}

#[derive(Debug, Default)]
pub struct ApplicationState {
    pub habit: Habit
}

pub fn update(state: &mut ApplicationState, message: Message){
    match message {
        Message::Close => {
                                std::process::exit(0);
                    },
        Message::SubmitHabitCreation() => {
                    createHabit::createHabit(state.habit.clone());
                    print!("Tasks where create successfully")
                },
        Message::NameInputChange(name) => state.habit.name = name,
        Message::DescInputChange(desc) => state.habit.desc = desc,
        Message::FrequencyInputChange(frequency) => {
                if frequency.len() <= 0 {
                    return;
                }
                let numbered_frequency = frequency.parse::<u8>().unwrap();
                state.habit.weekly_frequency = numbered_frequency
            },
        Message::GetHabits => println!("{:?}", listHabits()),
    }
}

pub fn view(state: &ApplicationState) -> Element<Message> {
    column![
        button("Close APP").on_press(Message::Close),

        text_input("Habit Name", &state.habit.name)
            .on_input(Message::NameInputChange),

        text_input("Description", &state.habit.desc)
            .on_input(Message::DescInputChange),

        text_input("How many times a week?", &state.habit.weekly_frequency.to_string())
            .on_input(Message::FrequencyInputChange),

        button("Submit").on_press(Message::SubmitHabitCreation()),
        button("Get habits").on_press(Message::GetHabits)

    ].spacing(10).into()
}