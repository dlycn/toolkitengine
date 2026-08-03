use bevy::prelude::*;

pub const PET_SPACING: f32 = 32.0;
pub const PET_MAX_SPEED: u32 = 100;
pub const PET_SIZE: u32 = 64;

#[derive(Resource, Default)]
pub struct SelectedPet(pub Option<Entity>);