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

pub struct Cpetstatus {
    pub atkdef:u64,
    pub hpspeed:u64,
    pub attr:u64,
}

//status
//u64::PA:u16,SA:u16,PD:u16,SD:u16
//u32::HP:u16,SP:u16
//u32::ATT:(u16,u16)

//detail
//hp:u32
//sp:u32
//pa:u32
//sa:u32
//pd:u32
//sd:u32
//flag:u32
//level:u16
//exp:u32
//skillpoint:u16
//skillused:[u16;6]
//personal:[u8;2]
//hpextra:u16
//critrate:f16
//critdamg:f16

#[derive(Component, Deref, DerefMut,Clone,Default)]
pub struct AnimationTimer(pub Timer);