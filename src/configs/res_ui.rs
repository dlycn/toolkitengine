use bevy::prelude::*;

pub const DEPTH_DELTA:f32 = 0.001;
pub const DEPTH_PET:f32 = 0.1;

#[derive(Resource)]
pub struct Ruilayer{
    pub pet_depth:f32,
}
impl Default for Ruilayer{
    fn default() -> Self {
        Self{
            pet_depth:0.1
        }
    }
}
