//databasemanager
use rusqlite::{Connection, Params, Result, Statement};

pub struct Table<T> {
    pub data: T,
}

impl<T> Table<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

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

    fn _desc_execute<P: Params>(&self, sql: &str, params: P, desc: &str) -> Result<usize> {
        let affected = self.conn.execute(sql, params)?;
        println!("{}", desc);
        Ok(affected)
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        self._desc_execute(sql, params, "日志")
    }

    fn _desc_prepare(&self, sql: &str, desc: &str) -> Result<Statement<'_>> {
        println!("{}", desc);
        self.conn.prepare(sql)
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        self._desc_prepare(sql, "游标")
    }
}
