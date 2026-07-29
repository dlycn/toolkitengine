use bevy::{color::palettes::css::{GREEN, RED}, prelude::*};

use crate::entities::{test,pet};

pub fn init(id:u32) -> impl Scene {
    bsn!{
    ImageNode{
        image:format!("imgs/Headers/{}.png",id),
        image_mode:NodeImageMode::Stretch,
    }
    Node{
        display:Display::Flex,
        left: Val::Percent(0.),
        top: Val::Percent(0.),
        height: Val::Px(64.),
        overflow: Overflow::scroll(),
        aspect_ratio: {Some(1.0)},
    }
    }
}

pub fn build() -> impl SceneList {
    bsn_list![
    init(5000)
    test::mesh2d(65536.0,RED),
    test::mesh2d(16384.0,GREEN),
    pet::new("Elaina Proctor",1),
    pet::selected("Renzo Hume",5000),
    pet::new("Zayna Nieves",2300),
    ]
}
