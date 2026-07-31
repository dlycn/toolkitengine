use bevy::prelude::*;
use crate::{components::*, configs::res_ui::DEPTH_PET};
pub fn new(name:&str,id:u32) -> impl Scene {
    let path = format!("imgs/Tachies/{}.png",id);
    bsn!{
    Cpet{name:{name.to_string()},maxspeed:100}
    Cbehavior{direction:Vec2::ONE,speed:16}
    Sprite{image: path,
    }
    Transform::from_xyz(0.0, 0.0, DEPTH_PET)}
}