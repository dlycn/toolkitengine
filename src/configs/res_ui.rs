use bevy::{prelude::*};

const EXP_MAX:u32 = 8;
const OFS_MAP:f32 = -1.0;
const OFS_PET:f32 = 0.1;

pub struct UIofs{
    pub map:f32,
    pub pet:f32,
}


/// ## offset for z-axis
/// ### default
/// - map = **-1.0**
/// - pet = **0.1**
pub const OFS:UIofs = UIofs{
    map:OFS_MAP,
    pet:OFS_PET,
};

/// - exponent for tile size
/// - default: **10**
pub const EXP_BASE:u32 = 10;

pub const EXP_AREA:u32 = EXP_BASE+EXP_MAX;
pub const STD_MAP:u32 = 2u32.pow(EXP_BASE);
pub const LOD_LEVELS:usize = 5;
pub const LOD_MAP:[u32;LOD_LEVELS] = [1,4,16,64,2u32.pow(EXP_MAX)];
pub const LOD_IDX:[f32;LOD_LEVELS] = [0.,1.95,7.85,31.5,127.0];


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
            map_areas:5,
            map_sides:STD_MAP*LOD_MAP[0],
            map_center:IVec2::ZERO,
            map_prepos:Vec::new(),
        }
    }
}
