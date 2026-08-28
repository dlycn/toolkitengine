use bevy::prelude::*;

pub const OFS_MAP:f32 = -1.0;
pub const OFS_PET:f32 = 0.1;
pub const EXP_BASE:u32 = 10;
pub const EXP_AREA:u32 = EXP_BASE+8;
pub const STD_MAP:u32 = 2u32.pow(EXP_BASE);
pub const LOD_LEVELS:usize = 4;
pub const LOD_MAP:[u32;LOD_LEVELS] = [1,4,16,64];
pub const LOD_IDX:[f32;LOD_LEVELS] = [0.25,1.95,7.85,31.5];


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
            map_areas:3,
            map_sides:STD_MAP*LOD_MAP[0],
            map_center:IVec2::ZERO,
            map_prepos:Vec::new(),
        }
    }
}
