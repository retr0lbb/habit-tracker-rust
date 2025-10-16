use std::sync::Mutex;
use postgres::{Client, NoTls};
use once_cell::sync::Lazy;


pub static DB: Lazy<Mutex<Client>> = Lazy::new(|| {
    let client = Client::connect("host=localhost user=docker password=docker dbname=habits", NoTls)
        .expect("Erro ao se conectar com o banco de dados");

    Mutex::new(client)
});


pub fn get() -> std::sync::MutexGuard<'static, Client>{
    DB.lock().unwrap()
}


pub fn setup(){
    let mut conn = get();
    conn.batch_execute(
        "
        DROP TABLE IF EXISTS completions;
        DROP TABLE IF EXISTS habits;

        CREATE TABLE IF NOT EXISTS habits (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            weekly_frequency INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS completions (
            id SERIAL PRIMARY KEY,
            habit_id INTEGER REFERENCES habits(id) ON DELETE CASCADE,
            completed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        ",
    ).expect("Erro ao criar as tabelas");

    println!("Tabela criada com sucesso")
}