use rusqlite::{Connection, Result};

pub fn connect()->Result<()>{
    let conn = Connection::open("habit-db.db")?;

    println!("Conectado com a base de dados");
    print!("{:?}", conn);

    let _ = conn.execute("CREATE TABLE IF NOT EXISTS habits(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        isComplete BOOLEAN NOT NULL DEFAULT 0
    )", []);

    println!("TABELA CRIADA COM SUCESSO");

    Ok(())
}