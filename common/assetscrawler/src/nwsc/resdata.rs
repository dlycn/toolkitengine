use std::collections::HashMap;

use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct PetData {
    pub id: u32,
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

#[derive(Serialize, Debug, Clone)]
pub struct PetSkin {
    pub id: u16,
    pub name: String,
    pub kind: String,
    pub bind: u32
}


pub struct Resfix {
    pub update: HashMap<(&'static str, &'static str), Vec<(String,String)>>,
    pub delete: Vec<(&'static str, &'static str)>,
}



pub trait Work {
    fn fix(&self)->Resfix{
        Resfix::default()
    }
}

impl Work for PetSkin {
    fn fix(&self) -> Resfix {
        let mut res = Resfix::default();
        res.update.entry(("id","kind"))
        .or_insert([1400771,1400775,1400792,1400798,1400805,1400813,1400818,1400832,1400833,1400834,1400854,1400644,1400644,1400645,1400653,1400654,1400655,1400658,1400659,1400660,1400692,1400740,1400749]
            .iter().map(|i|{(i.to_string(),"传说".to_string())}).collect::<Vec<(String,String)>>());
        res.update.get_mut(&("id","kind")).unwrap().push(("1400310".into(),"经典".into()));
        res
    }
}

impl Work for PetData {
    fn fix(&self) -> Resfix {
        let mut res = Resfix::default();

        res.delete.push(("id", "3228"));
        res.delete.push(("id", "3229"));
        res.delete.push(("id", "3230"));

        res
    }
}

impl Default for PetSkin {
    fn default() -> Self {
        Self { id: 0, 
            name: "未知".into(), 
            kind: "未知类型".into(),
            bind: 0
        }
    }
}

impl Default for PetData {
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