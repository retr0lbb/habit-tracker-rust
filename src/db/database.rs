use std::sync::Mutex;

use rusqlite::{Connection};
use once_cell::sync::Lazy;


pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("habit-db.db").expect("Erro ao ler banco de dados");

    Mutex::new(conn)
});


pub fn get() -> std::sync::MutexGuard<'static, Connection>{
    DB.lock().unwrap()
}


pub fn setup(){
    let conn = get();

    conn.execute( "CREATE TABLE IF NOT EXISTS habits (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                desc TEXT,
                is_complete BOOLEAN NOT NULL
            )", []).unwrap();

    println!("Tabela criada com sucesso")
}