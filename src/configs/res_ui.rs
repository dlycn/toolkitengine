use bevy::prelude::*;

pub const OFS_MAP:f32 = -1.0;
pub const OFS_PET:f32 = 0.1;
pub const STD_MAP:u32 = 512;
pub const LOD_LEVELS:usize = 5;
pub const LOD_MAP:[u32;LOD_LEVELS] = [1,4,12,36,48];
pub const LOD_IDX:[f32;LOD_LEVELS] = [0.,1.5,6.,18.,32.];


#[derive(Resource)]
pub struct Ruilayer{
    pub lod_level:u8,
    pub map_areas:u8,
    pub map_sides:u32,
    pub map_center:IVec2,
    pub map_prepos:Vec<IVec2>,
}
impl Default for Ruilayer{
    fn default() -> Self {
        Self{
            lod_level:0,
            map_areas:4,
            map_sides:STD_MAP*LOD_MAP[0],
            map_center:IVec2::ZERO,
            map_prepos:Vec::new(),
        }
    }
}
