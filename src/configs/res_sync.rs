use bevy::prelude::*;
#[derive(Resource)]
pub struct RControl{pub speed:f32}
impl Default for RControl {
    fn default() -> Self {RControl{
        speed:5.0
    }}}
