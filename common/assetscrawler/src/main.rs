mod dbmg;
mod nwsc;

use rusqlite::{Connection, Result};
use dbmg::{Table,Databasemanager};


#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {

    let path = "./common/assetscrawler/test.db";
    let conn = Connection::open(path)?;
    let dbm = Databasemanager::new(String::from(path),conn);
    print!("{}",dbm.dbmeta());

    if let Ok(text) = nwsc::readurl("https://www.rust-lang.org"){
        println!("{}",text)
    }

    dbm.execute(
        "CREATE TABLE IF NOT EXISTS Person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Stven".to_string(),
        data: None,
    };
    let table = Table::new(&me);
    dbm.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;
    println!("table.data.id:{}",table.data.id);

    let mut stmt = dbm.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}