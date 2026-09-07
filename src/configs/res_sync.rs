use bevy::prelude::*;
pub const INDEX_MAX:i8 = 7;
pub const INDEX_MIN:i8 = -1;
#[derive(Resource)]
pub struct RControl {
    pub speed: f32,
    pub scale: f32,
    pub rate: f32,
    pub maxfactor: f32,
    pub minfactor: f32,
}
impl Default for RControl {
    fn default() -> Self {
        RControl {
            speed: 512.0,
            scale: 2.0,
            rate: -0.5,
            maxfactor: 2f32.powf(INDEX_MAX as f32),
            minfactor: 2f32.powf(INDEX_MIN as f32),
        }
    }
}
#[derive(Resource)]
pub struct RGobalLocation {
    pub pos: Vec3,
    pub index: IVec2,
}
impl Default for RGobalLocation {
    fn default() -> Self {
        RGobalLocation {
            pos: Vec3::ZERO,
            index: IVec2::ZERO,
        }
    }
}
