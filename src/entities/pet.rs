use bevy::{prelude::*};
use crate::{components::*, configs::{res_pet::PET_MAX_SPEED, res_ui::DEPTH_PET}};
pub fn new(name:&str,id:u32) -> impl Scene {
    let path = format!("imgs/Tachies/{}.png",id);
    bsn!{
    Cpet{name:{name.to_string()},maxspeed:PET_MAX_SPEED}
    Cbehavior{direction:Vec2::ZERO,speed:0}
    Sprite{image: path,
    }
    Transform::from_xyz(0.0, 0.0, DEPTH_PET)}
}