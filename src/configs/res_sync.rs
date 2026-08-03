use bevy::prelude::*;



#[derive(Resource)]
pub struct RControl {
    pub speed: f32,
    pub scale: f32,
    pub rate: f32,
    pub lodsprite: f32,
    pub maxfactor: f32,
    pub minfactor: f32,
}
impl Default for RControl {
    fn default() -> Self {
        RControl {
            speed: 512.0,
            scale: 2.0,
            rate: -0.5,
            lodsprite: 4.0,
            maxfactor: 64.0,
            minfactor: 0.5,
        }
    }
}
