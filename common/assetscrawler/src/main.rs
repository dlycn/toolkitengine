mod dbmg;
mod nwsc;
use dbmg::{Databasemanager, Table};
use rusqlite::{Connection, Result};
use serde::*;

#[derive(Serialize, Debug, Default, Clone)]
struct Person {
    name: String,
    data: u8,
}

fn main() -> Result<()> {
    let path = "./common/assetscrawler/assets.db";
    let conn = Connection::open(path)?;
    let dbm = Databasemanager::new(String::from(path), conn);
    print!("{}", dbm.dbmeta());
    let text = nwsc::readurl("https://www.rust-lang.org");
    println!("{}", text.len());

    let me = Person {
        name: "Stven".to_string(),
        data: u8::MAX,
    };
    let table = Table::new(me.clone());
    let dbtsys = table.sys().pk("name").go();
    let dbtupd = table.update();
    let dptdop =table.drop();
    dbm.execute(&dbtsys, ())?;
    dbm.execute(&dbtupd, (&me.name, &me.data))?;

    println!("name:{}", table.data.name);

    let mut stmt = dbm.prepare("SELECT name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            name: row.get(0)?,
            data: row.get(1)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }

    Ok(())
}
