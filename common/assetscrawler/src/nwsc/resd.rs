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

pub trait Work {
    fn fix(self)->bool;
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
