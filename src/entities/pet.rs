use crate::{
    components::*, configs::{res_pet::PET_MAX_SPEED}, events::pets::ESelectPet,
};
use bevy::{gltf::gltf_ext::scene, image::TextureAtlasTemplate};
use bevy::prelude::*;
use crate::configs::res;
use res::layer::*;
use res::pet::{PET_EXP,STD_DIFF,PET_DIFF};



pub fn stdpet(
    texture: Handle<Image>,
    texture_atlas_layout:Handle<TextureAtlasLayout>,
    size:Vec2,
    pos:Vec2,
) -> impl Scene {
    println!("{:?}",size);
    bsn!{
            Sprite{
                image:texture,
                texture_atlas:Option::Some(TextureAtlasTemplate{layout:texture_atlas_layout,index:0}),
                custom_size:Option::Some(size),
            }
            AnimationIndices { first: 0, last: 26 }
            Transform::from_xyz(pos.x, pos.y, OFS_PET)
            Cpet{name:{"圣灵谱尼".to_string()},maxspeed:PET_MAX_SPEED}
            Cbehavior{direction:Vec2::ZERO,speed:0,gobalpos:IVec2::ZERO}
            AnimationTimer(Timer::from_seconds(5.0/128.0, TimerMode::Repeating))      
            
    }
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("imgs/5000.basisu.ktx2");
    let size = [16,14];
    let array =UVec2::new(size[0]<<STD_DIFF,size[1]<<STD_DIFF);
    let layout = TextureAtlasLayout::from_grid(array, 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let pet = stdpet(texture, texture_atlas_layout, Vec2::new((size[0]<<PET_DIFF) as f32,(size[1]<<PET_DIFF) as f32), Vec2::ZERO);

    let pet = commands.spawn_scene(pet).id();
    commands.trigger(ESelectPet{entity:pet});

}