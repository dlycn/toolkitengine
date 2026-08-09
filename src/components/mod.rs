use bevy::prelude::*;

#[derive(Component,Default,Clone)]
pub struct Cpet{pub name:String, pub maxspeed:u32}
#[derive(Component,Default,Clone)]
pub struct Cbehavior{
    pub direction:Vec2,
    pub speed:u32,
}

#[derive(Component,Default,Clone, Copy)]
pub struct Cselect;

#[derive(Component,Default,Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut,Clone,Default)]
pub struct AnimationTimer(pub Timer);