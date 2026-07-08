//databasemanager

mod table;

use rusqlite::{Connection, Params, Result, Statement};

pub use self::table::Table;

pub struct Databasemanager {
    path: String,
    conn: Connection,
}

impl Databasemanager {
    pub fn dbmeta(&self) -> String {
        format!("path\t{}", &self.path)
    }

    pub fn new(path: String, conn: Connection) -> Self {
        Self { path, conn }
    }

    pub fn desc_execute<P: Params>(&self, sql: &str, params: P, desc: &str) -> Result<usize> {
        let affected = self.conn.execute(sql, params)?;
        if desc != String::new(){println!("{}", desc)};
        Ok(affected)
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        self.desc_execute(sql, params, "")
    }

    pub fn desc_prepare(&self, sql: &str, desc: &str) -> Result<Statement<'_>> {
        println!("{}", desc);
        self.conn.prepare(sql)
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        self.desc_prepare(sql, "")
    }
}
