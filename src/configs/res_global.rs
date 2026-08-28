use bevy::{prelude::*};
use crate::configs::material;
// #[derive(Resource)]
// pub struct Rcursorpos(pub Option<Vec2>);


/// # 资源：标志位
/// 1. ...
/// 
#[derive(Resource,Default)]
pub struct Rbitflag(pub u32);

#[derive(Resource)]
pub struct RHandle{
    pub font: Option<Handle<Font>>,
    pub map_material: Option<Handle<material::MapMaterial>>,
    pub hp_material: Option<Handle<material::HpMaterial>>,
}

#[derive(Resource,Clone)]
pub struct RAreaHandle{
    pub areas_material: [Option<Handle<material::AreaMaterial>>;25],
}

impl Default for RHandle{
    fn default() -> Self {
        Self {
            font: None,
            map_material: None,
            hp_material: None,
        }
    }
}

impl Default for RAreaHandle{
    fn default() -> Self {
        Self {
            areas_material: [const { Option::None::<bevy::prelude::Handle<material::AreaMaterial>> };25],
        }
    }
}
