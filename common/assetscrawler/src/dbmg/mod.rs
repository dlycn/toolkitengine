//databasemanager

pub mod tfm;

use std::fs;

use serde::*;

use rusqlite::{Connection, Params, Result, Statement};

use tfm::Table;

// 1. 数据库构建器（去掉泛型 T，只管理连接）
pub struct Dbmbuilder {
    pub manager: Databasemanager,  // 持有所有权
}

impl Dbmbuilder {

    pub fn connect(path: String) -> Result<Self> {
        let conn = Connection::open(&path)?;  // 传引用，不消耗 path
        Ok(Self {
            manager: Databasemanager::new(path, conn)
        })
    }

    // 核心：on 方法不持有表，而是返回一个“临时工” TableOperator
    pub fn on<'a, T: Serialize + 'static>(&'a mut self, table: &'a Table<T>) -> TableOperator<'a, T> {
        TableOperator {
            db_builder: self,    // 借用构建器（可变）
            table,
            primary_key: None,
            rowname: None
        }
    }

    // 内部执行方法（供 TableOperator 调用）
    pub(crate) fn execute(&self, sql: &str,desc: &str) -> Result<usize> {
        self.manager.desc_execute(sql, [],desc)
    }

    pub(crate) fn run<P: Params>(&self, sql: &str,params: P,desc: &str) -> Result<usize> {
        self.manager.desc_execute(sql,params,desc)
    }    

    pub fn transaction(&self, mode: bool) -> Result<usize> {
        let (sql, desc) = if mode {
            ("BEGIN TRANSACTION", "开启数据库事务")
        } else {
            ("COMMIT", "提交数据库事务")
        };
        self.execute(sql, desc)
    }

}

pub struct TableOperator<'a, T> {
    db_builder: &'a mut Dbmbuilder,  // 对主构建器的可变借用
    table: &'a Table<T>,
    primary_key: Option<String>,
    rowname: Option<String>
}

impl<'a, T: Serialize + 'static> TableOperator<'a, T> {
    pub fn pk(mut self, key: &str) -> Self {
        self.primary_key = Some(key.to_string());
        self
    }

    pub fn row(mut self, row: &str) -> Self {
        let r = String::from(row);
        if self.table.table_rown().contains(&r){
            self.rowname = Some(r);}
    self
    }

    pub fn create(self) -> &'a mut Dbmbuilder {
        // 1. 先获取表构建器
        let mut builder = self.table.sys();
        // 2. 如果有主键设定，则调用 pk 方法
        if let Some(ref pk) = self.primary_key {
            builder = builder.pk(pk);  // pk 是 &String，自动解引用为 &str
        }
        
        // 3. 生成 SQL 并执行
        let sql = builder.go();
        let desc = format!("创建数据表{}",self.table.name);
        let _ = self.db_builder.execute(&sql,desc.as_str());
        
        // 4. 归还构建器的可变引用
        self.db_builder
    }

    // 删表
    pub fn drop(self) -> &'a mut Dbmbuilder {
        let sql = self.table.drop();
        let desc = format!("删除数据表{}",self.table.name);
        let _ = self.db_builder.execute(&sql,desc.as_str());
        self.db_builder
    }

    pub fn update<P: Params+ std::fmt::Debug>(self,pkn: String,params:P) -> &'a mut Dbmbuilder {
            let pk = self.primary_key.unwrap_or_default();
            let rn = self.rowname.unwrap_or_default();        
        if !pk.is_empty(){
            let sql = self.table.fixupdate(&pk,&rn, &pkn);
            let desc = format!("更新数据表{}，条目：[{}={}]，{}={:?}",self.table.name,pk,pkn,rn,params);
            let _ = self.db_builder.run(&sql,params,desc.as_str());
        }else{println!("unexpect primary_key:{},rowname:{}",pk,rn);}
        self.db_builder
    }

    pub fn delete(self,pkn: String) -> &'a mut Dbmbuilder {
            let pk = self.primary_key.unwrap_or_default();     
        if !pk.is_empty(){
            let sql = self.table.delete(&pk,&pkn);
            let desc = format!("删除数据表{}，条目：[{}={}]",self.table.name,pk,pkn);
            let _ = self.db_builder.execute(&sql,desc.as_str());
        }else{println!("unexpect primary_key:{}",pk);}
        self.db_builder
    }

    // 插入数据
    pub fn append<P: Params>(self,lines:usize,params:P,cut:Option<usize>) -> &'a mut Dbmbuilder {
        let desc = format!("插入数据表{}，追加行数{}",self.table.name,lines);
        let sql = self.table.allinsert(lines,cut);
        fs::write("db.log", &sql).unwrap();
        let _ = self.db_builder.run(&sql,params,desc.as_str());
        self.db_builder
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
        Self { path, conn}
    }


    pub fn desc_execute<P: Params>(&self, sql: &str, params: P, desc: &str) -> Result<usize> {
        let affected = self.conn.execute(sql, params).expect(sql);
            if !desc.is_empty() {println!("{}", desc);}
            Ok(affected)
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        self.desc_execute(sql, params, "")
    }

    pub fn desc_prepare(&self, sql: &str, desc: &str) -> Result<Statement<'_>> {
        if !desc.is_empty() {println!("{}", desc);}
        self.conn.prepare(sql)
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        self.desc_prepare(sql, "")
    }
}
