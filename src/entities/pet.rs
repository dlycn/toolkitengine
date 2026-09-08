use crate::{
    components::*, configs::{res_pet::PET_MAX_SPEED}, events::pets::ESelectPet,
};
use bevy::{image::TextureAtlasTemplate};
use bevy::prelude::*;
use crate::configs::res;
use crate::events::pets::ECreatePet;
use res::layer::*;
use res::pet::{STD_DIFF,PET_DIFF,CreatedPet};



pub fn stdpet(
    texture: Handle<Image>,
    texture_atlas_layout:Handle<TextureAtlasLayout>,
    indice:usize,
    name:String,
    size:Vec2,
    pos:Vec2,
) -> impl Scene {
    bsn!{
            Sprite{
                image:texture,
                texture_atlas:Option::Some(TextureAtlasTemplate{layout:texture_atlas_layout,index:0}),
                custom_size:Option::Some(size),
            }
            AnimationIndices { first: 0, last: indice }
            Transform::from_xyz(pos.x, pos.y, OFS.pet)
            Cpet{name:name,maxspeed:PET_MAX_SPEED}
            Cbehavior{direction:Vec2::ZERO,speed:0,gobalpos:IVec2::ZERO}
            AnimationTimer(Timer::from_seconds(5.0/128.0, TimerMode::Repeating))      
            
    }
}


pub fn mksetup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("imgs/5000.basisu.ktx2");
    let size = [16,14];
    let array =UVec2::new(size[0]<<STD_DIFF,size[1]<<STD_DIFF);
    let layout = TextureAtlasLayout::from_grid(array, 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let pet = stdpet(texture, texture_atlas_layout, 26, "圣灵谱尼".to_string(), Vec2::new((size[0]<<PET_DIFF) as f32,(size[1]<<PET_DIFF) as f32), Vec2::ZERO);

    let pet = commands.spawn_scene(pet).id();
    commands.trigger(ESelectPet{entity:pet});

}

pub fn pet_select(
    mut commands: Commands,
    respet: Res<CreatedPet>) {
    if respet.0.is_some(){
        let pet = respet.0.unwrap();
        commands.trigger(ESelectPet{entity:pet});
    }
}


pub fn setup(
    mut commands: Commands,
) {
    let mut idnum: Vec<([usize; 2], fn(usize) -> Vec2)> = Vec::new();
    
    idnum.push(([5000, 100000], |i| Vec2::new(i as f32, 0.0)));
    idnum.push(([3414, 100000], |i| Vec2::new(i as f32, 0.0)));

    commands.trigger(ECreatePet{idnum});
}