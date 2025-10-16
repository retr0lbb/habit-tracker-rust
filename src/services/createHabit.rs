use rusqlite::params;
use rusqlite::Result;

use crate::{app::Habit, db::database::get};

pub fn createHabit(habit: Habit){
    let mut database_instance= get();

    database_instance.execute(
        "INSERT INTO habits 
        (name, description, weekly_frequency) 
        VALUES 
        ($1, $2, $3)", 
    &[&habit.name, &habit.desc, &(habit.weekly_frequency as i32)],
    ).expect("Erro ao inserir dados ");
}

pub fn listHabits() -> Result<Vec<Habit>>{
    let mut database_instance = get();

    let mut habits: Vec<Habit> = Vec::new();

    for row in database_instance.query("SELECT id, name, description, weekly_frequency FROM habits", &[]).unwrap() {
        print!("cade a coluna? {:?}", row);

       let habit = Habit {
        id: row.get(0),
        name: row.get(1),
        desc: row.get(2),
        weekly_frequency: row.get::<_, i32>(3) as u8
       };

       habits.push(habit);
    }

    Ok(habits)
}