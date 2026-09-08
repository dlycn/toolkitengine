use bevy::prelude::*;

const AYS_EXP: u8 = 4;
const STD_EXP: u8 = 8;



pub const PET_SPACING: f32 = 32.0;
pub const PET_MAX_SPEED: u32 = 1200;
pub const PET_EXP: u8 = 7;

pub const STD_DIFF: u8 = STD_EXP - AYS_EXP;
pub const PET_DIFF: u8 = PET_EXP - AYS_EXP;

#[derive(Resource, Default)]
pub struct SelectedPet(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct CreatedPet(pub Option<Entity>);