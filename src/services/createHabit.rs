use rusqlite::params;
use rusqlite::Result;

use crate::{app::Habit, db::database::get};

pub fn createHabit(habit: Habit){
    let database_instance: std::sync::MutexGuard<'static, rusqlite::Connection> = get();

    database_instance.execute(
        "INSERT INTO habits 
        (name, desc, weekly_frequency) 
        VALUES 
        (?1, ?2, ?3)", 
    params![habit.name, habit.desc, habit.weekly_frequency]).expect("Erro ao inserir dados ");
}

pub fn listHabits() -> Result<Vec<Habit>>{
    let database_instance = get();
    
    let mut stmt = database_instance.prepare(
        "SELECT id, name, desc, weekly_frequency FROM habits"
    ).unwrap();


    let habits_iter = stmt.query_map([], |row| {
        Ok(Habit {
            id: row.get(0)?,
            name: row.get(1)?,
            desc: row.get(2)?,
            weekly_frequency: row.get(3)?,
        })
    })?;

    let habits: Vec<Habit> = habits_iter.filter_map(|r|r.ok()).collect();

    Ok(habits)
}