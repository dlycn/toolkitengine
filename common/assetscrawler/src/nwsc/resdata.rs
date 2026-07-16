use std::collections::HashMap;

use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct Role {
    pub id: u16,
    pub name: String,
    pub attr: String,
    pub gender: String,
    pub pa:u32,
    pub sa:u32,
    pub pd:u32,
    pub sd:u32,
    pub sp:u32,
    pub hp:u32,
    pub stats:u32,
    pub post:String
}

pub struct Resfix {
    pub update: HashMap<(&'static str, &'static str), Vec<(&'static str, &'static str)>>,
    pub delete: Vec<(&'static str, &'static str)>,
}



pub trait Work {
    fn fix(self)->Resfix;
}

impl Work for Role {
    fn fix(self) -> Resfix {
        let mut res = Resfix::default();

        // let k = update
        //     .entry(("id", "attr"))
        //     .or_insert_with(Vec::new);
        // k.push(("3228", "王系"));

        res.delete.push(("id", "3228"));
        res.delete.push(("id", "3229"));
        res.delete.push(("id", "3230"));

        res
    }
}

impl Default for Role {
    fn default() -> Self {
        Self { id: 0, 
            name: "未知".into(), 
            attr: "未知属性".into(), 
            gender: "无性别".into(), 
            pa: 0, sa: 0, pd: 0, sd: 0,
            sp: 0, hp: 0, stats: 0,
            post:"".into() }
    }
    
}

impl Default for Resfix {
    fn default() -> Self {
        Self { update: HashMap::new(), delete: Vec::new() }
    }
    
}