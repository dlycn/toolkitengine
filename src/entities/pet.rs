use bevy::prelude::*;
use crate::components::*;
pub fn new(name:&str,id:u32) -> impl Scene {
    let path = format!("imgs/Tachies/{}.png",id);
    bsn!{
    Cpet{name:{name.to_string()}}
    Sprite{image: path,
    }}
}
pub fn selected(name:&str,id:u32) -> impl Scene {
    bsn!{
        new(name,id)
        Cselect
    }
}