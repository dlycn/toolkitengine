use bevy::prelude::*;
use crate::components::Cselect;
pub fn init() -> impl Scene {
    bsn!{
        Camera2d
        Camera{
            clear_color: ClearColorConfig::Custom(Color::NONE),
        }
        Transform
        Cselect
    }
}

pub fn build() -> impl SceneList {
    bsn_list![
    init(),
    ]
}