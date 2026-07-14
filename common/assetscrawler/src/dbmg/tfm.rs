
use serde::*;
use serde_json;
use std::{any::type_name, ops::Index};

fn matchdbtype(v: serde_json::Value)->(String,DBtype) {
    match v {
        serde_json::Value::Null => ("NULL".to_string(),DBtype::DBnull),
        serde_json::Value::Bool(bool) => (bool.to_string(),DBtype::DBint),
        serde_json::Value::Number(num)=>(num.to_string(),
        if num.is_f64(){DBtype::DBreal}else {DBtype::DBint}),
        serde_json::Value::String(str) => (format!("'{}'",str),DBtype::DBstr),
        serde_json::Value::Array(_) => ("x''".to_string(),DBtype::DBblob),
        serde_json::Value::Object(_) => panic!("Abnormal data type"),
    }
}

pub struct CreateTableBuilder<'a, T> {
    table: &'a Table<T>,
    primary_key: Option<String>,
}
impl<'a, T: Serialize + 'static> CreateTableBuilder<'a, T> {
    fn new(table: &'a Table<T>) -> Self {
        Self {
            table,
            primary_key: None,
        }
    }

    /// 设置主键字段名
    pub fn pk(mut self, key: &str) -> Self {
        self.primary_key = Some(key.to_string());
        self
    }

    pub fn go(self) -> String {
        let sys = self.table.table_meta();
        let ofbeg = format!("CREATE TABLE IF NOT EXISTS {}", sys.name);
        
        // 提前取出主键引用，若无则设为 None
        let pk_ref = self.primary_key.as_ref(); // 或者 as_deref()
        
        let field_strs: Vec<String> = sys.fields
            .iter()
            .map(|row| {
                let mut s = row.gotext();
                // 如果有主键且字段名匹配，则追加
                if let Some(pk) = pk_ref {
                    if row.0 == *pk {
                        s.push_str(" PRIMARY KEY");
                    }
                }
                s
            })
            .collect();
        
        let fields_part = field_strs.join(", ");
        format!("{}({})", ofbeg, fields_part)
    }
}


pub struct Table<T> {
    pub data: T,
    pub name: String,
    pub rows: Vec<String>
}

impl<T: Serialize + 'static> Table<T> {
    // 构造时传入实例化的结构体
    pub fn new(data: T) -> Self {
        let name = type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("")
            .to_string();
        let rows: Vec<String> = Vec::new();
        Self { data, name, rows } // 假设结构体中有一个名为 meta 的字段
    }

    pub fn anchor(mut self,coln:String)->Self{
        let mut rows:   Vec<String> = self.table_rown();
        let upk = rows.index(0);
        if rows.contains(&coln){
            rows = vec![upk.clone(),coln]
        }
        self.rows=rows;
        self
    }

    pub fn table_name(&self) -> String {
        self.name.clone()
    }

    pub fn table_meta(&self) -> Tablemeta{
        self.build()
    }

    pub fn table_rown(&self) -> Vec<String> {
        let fields = Self::field(&self.data);
        let rows:   Vec<String> = fields.keys().cloned().collect();
        rows
    }

    pub fn table_field(&self) -> Vec<(String, serde_json::Value)> {
        let obj =Self::field(&self.data);
        let opt=obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        opt
    }

    fn field(data: &T) -> serde_json::Map<String,serde_json::Value> {
        let value = serde_json::to_value(data).unwrap();
        match value {
            serde_json::Value::Object(map) => map,
            _ => panic!("T 必须是一个结构体"),
        }
    }

    pub fn  update(&self,lines:usize) ->String{
        let rows:   Vec<String> = if self.rows.is_empty(){self.table_rown()}else{self.rows.clone()};
        let coins: Vec<String>= rows.iter().map(|_|format!("?")).collect();
        let bath = format!("({})", coins.join(","));
        let coinlines = vec![bath; lines].join(", ");
        format!("INSERT OR REPLACE INTO {} ({}) VALUES {}",self.name,rows.join(", "), coinlines)
    }

    pub fn drop(&self) ->String{
        format!("DROP TABLE IF EXISTS {};",self.name)
    }

    fn build(&self) -> Tablemeta {
        let fields = self.table_field()
            .into_iter()
            .map(|(name, val)| {
                let (def, db_type) = matchdbtype(val);
                Rowmeta(name,db_type,def)
            })
            .collect();

        Tablemeta {
            name: self.table_name(),
            fields,
        }
    }

    //返回构建器模式
    pub fn sys(&self) -> CreateTableBuilder<'_, T> {
        CreateTableBuilder::new(self)
    }

}

#[derive(Debug)]
struct Rowmeta(String, DBtype, String);

impl Rowmeta {
    fn gotext(&self) -> String {
        format!("{} {} DEFAULT {} NOT NULL", self.0, self.1.gotext(), self.2)
    }
}

#[derive(Debug)]
pub struct Tablemeta {
    name: String,
    fields: Vec<Rowmeta>, // rowname,rowtype
}

#[derive(Debug)]
enum DBtype {
    DBreal,
    DBint,
    DBstr,
    DBblob,
    DBnull,
}

impl DBtype {
    fn gotext(&self) -> &'static str {
        match self {
            DBtype::DBreal => "Real",
            DBtype::DBint => "Integer",
            DBtype::DBstr => "TEXT",
            DBtype::DBblob => "Blob",
            DBtype::DBnull => "Null",

        }
    }
}

