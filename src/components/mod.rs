use bevy::prelude::*;

mod flags;

#[derive(Component,Default,Clone, Copy)]
pub struct Cselect;

pub mod ui{
    pub use crate::components::flags::*;
}


#[derive(Component,Default,Clone)]
pub struct Cpet{pub name:String, pub maxspeed:u32}
#[derive(Component,Default,Clone)]
pub struct Cbehavior{
    pub direction:Vec2,
    pub speed:u32,
    pub gobalpos:IVec2,
}

#[derive(Component,Default,Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component,Default,Clone)]
pub struct PetConfig {
    pub texture_size:[u32;2],
    pub pet_name:String,
    pub pet_indice:usize
}

#[derive(Component, Deref, DerefMut,Clone,Default)]
pub struct AnimationTimer(pub Timer);