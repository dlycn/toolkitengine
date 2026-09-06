use crate::{
    components::*, configs::{res_pet::PET_MAX_SPEED, res_ui::DEPTH_PET}, events::pets::ESelectPet,
};
use bevy::image::TextureAtlasTemplate;
use bevy::prelude::*;
<<<<<<< HEAD
use res::pet::*;
use res::layer::*;

use super::global;


pub fn stdbundle(
    texture: Handle<Image>,
    texture_atlas_layout:Handle<TextureAtlasLayout>,
) -> impl Scene {
    bsn!{
            Sprite{
                image:texture,
                texture_atlas:Option::Some(TextureAtlasTemplate{layout:texture_atlas_layout,index:0}),
            }
            AnimationIndices { first: 0, last: 26 }
            Transform::from_xyz(0.0, 0.0, OFS_PET)
            Cpet{name:{"圣灵谱尼".to_string()},maxspeed:PET_MAX_SPEED}
            Cbehavior{direction:Vec2::ZERO,speed:0,gobalpos:IVec2::ZERO}
            AnimationTimer(Timer::from_seconds(5.0/128.0, TimerMode::Repeating))      
            
    }
=======
pub fn new(name: &str, id: u32) -> impl Scene {
    let path = format!("imgs/Tachies/{}.png", id);
    bsn! {
    Cpet{name:{name.to_string()},maxspeed:PET_MAX_SPEED}
    Cbehavior{direction:Vec2::ZERO,speed:0}
    Transform::from_xyz(0.0, 0.0, DEPTH_PET)
    tachie(path)}
}
pub fn tachie(path: String) -> impl Scene {
    bsn! {Sprite{image: path}}
>>>>>>> parent of e94b3f5 (pass)
}

pub fn statebundle(text:String) -> impl Scene {
    bsn!{Text2d::new(text)
    global::std_font(1.)
    Visibility::Hidden}
}

pub fn stdpet(texture: Handle<Image>,
    texture_atlas_layout:Handle<TextureAtlasLayout>) -> impl Scene {
    bsn!{
        stdbundle(texture, texture_atlas_layout)
        Children[
            statebundle("圣灵谱尼".to_string()),
        ]
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("imgs/3414.basisu.ktx2");
    let size = [16,14];
    let layout = TextureAtlasLayout::from_grid(UVec2::from_array(size.map(|x|x*(256/16))), 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let pet = commands.spawn_scene(stdpet(texture, texture_atlas_layout)).id();
    commands.trigger(ESelectPet{entity:pet});

}