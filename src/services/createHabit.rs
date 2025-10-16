use rusqlite::params;

use crate::{app::Habit, db::database::get};

pub fn createHabit(habit: Habit){
    let database_instance = get();

    database_instance.execute(
        "INSERT INTO habits 
        (name, desc, is_completed) 
        VALUES 
        (?1, ?2, ?3)", 
    params![habit.name, habit.desc, 0]).expect("Erro ao inserir dados ");
}