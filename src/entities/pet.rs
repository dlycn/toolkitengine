use crate::{
    components::*, 
    configs::res,
    events::pets::ESelectPet,
};
use bevy::prelude::*;
use res::pet::*;
use res::layer::*;
use res::global::*;


pub fn stdbundle(
    texture: Handle<Image>,
    texture_atlas_layout:Handle<TextureAtlasLayout>,
) -> (Sprite,AnimationIndices,Transform,Cpet,Cbehavior,AnimationTimer){
    let animation_indices = AnimationIndices { first: 0, last: 26 };
    let sprite = Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        );
    (
            sprite,
            animation_indices,
            Transform::from_xyz(0.0, 0.0, OFS_PET),
            Cpet{name:{"圣灵谱尼".to_string()},maxspeed:PET_MAX_SPEED},
            Cbehavior{direction:Vec2::ZERO,speed:0},
            AnimationTimer(Timer::from_seconds(5.0/128.0, TimerMode::Repeating)),      
            
    )
}

pub fn statebundle(text:String,fontfile:Handle<Font>) -> (Text2d,TextFont,TextColor,Visibility) {
    (Text2d::new(text),
    TextFont {
        font: FontSource::Handle(fontfile),
        font_size: FontSize::Px(48.0),
        style: FontStyle::Italic,
        ..Default::default()
    },
    TextColor(Color::WHITE),
    Visibility::Hidden)
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fontfile: Res<RHandle>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("imgs/image.basisu.ktx2");
    let size = [16,14];
    let layout = TextureAtlasLayout::from_grid(UVec2::from_array(size.map(|x|x*(256/16))), 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let pet = commands.spawn(stdbundle(texture,texture_atlas_layout)).with_child(statebundle("圣灵谱尼".to_string(),fontfile.font.as_ref().unwrap().clone())).id();
    commands.trigger(ESelectPet{entity:pet});

}

