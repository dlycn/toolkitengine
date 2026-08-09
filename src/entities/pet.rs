use crate::{
    components::*, configs::{res_pet::PET_MAX_SPEED, res_ui::DEPTH_PET}, events::pets::ESelectPet,
};
use bevy::prelude::*;
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
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("imgs/output.basisu.ktx2");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(967, 902), 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 26 };
    let sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        );

    let pet = commands.spawn((
        sprite,
        animation_indices,
        Transform::from_xyz(0.0, 0.0, DEPTH_PET),
        Cpet{name:{"圣灵谱尼".to_string()},maxspeed:PET_MAX_SPEED},
        Cbehavior{direction:Vec2::ZERO,speed:0},
        AnimationTimer(Timer::from_seconds(5.0/128.0, TimerMode::Repeating)),      
    )).id();
    commands.trigger(ESelectPet{entity:pet});

}

