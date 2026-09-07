use bevy::prelude::*;
use super::configs::{material,res,Setting};
use super::components::*;
use super::events::*;
use crate::events::setwindows::*;
use res::{hello,global,sync,layer};

pub mod kmcontrol;

mod hud;

pub mod apply{
    pub use crate::scripts::hud::font_apply as font;
    pub mod shaders{
        pub use crate::scripts::hud::global_shaders_apply as global;
        pub use crate::scripts::hud::area_shaders_apply as area;
    }
}

pub fn res_insert(mut commands: Commands){
    commands.insert_resource(sync::RControl::default());
    commands.insert_resource(global::RHandle::default());
    commands.insert_resource(global::Rbitflag::default());
    commands.insert_resource(global::RAreaHandle::default());
}

pub fn res_preload(asset_server: Res<AssetServer>,
    mut initres: ResMut<global::RHandle>,
    mut initareas_material: ResMut<Assets<material::AreaMaterial>>,
    mut ruilayer: ResMut<layer::Ruilayer>,
) {
    initres.font = Some(asset_server.load("fonts/source han sans.otf"));
    let area_material =material::AreaMaterial { color: LinearRgba::GREEN };
    let area_iter = (0..ruilayer.map_areas.pow(2) as i32).map(|i| IVec2::new(i%ruilayer.map_areas as i32,i/ruilayer.map_areas as i32));
    initareas_material.add(area_material);
    ruilayer.map_prepos=Vec::from_iter(area_iter);
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn greet_people(mut commands: Commands,time: Res<Time>, mut timer: ResMut<hello::RGreetTimer>, query: Query<&Cpet>) {
    if timer.tick(time.delta()){
        commands.trigger(EUpdatePet {target:"Elaina Proctor".to_string(),parameter:"Elaina Hume".to_string()});
    }
    if timer.0.just_finished() {
        for pet in &query {
            println!("hello {}!", pet.name);
        }
    }
}

pub fn pet_behavior(querybehavior:Query<(&mut Cbehavior,&mut Transform),With<Cpet>>,time: Res<Time>) {
    if querybehavior.is_empty() {return;};
    for pet in  querybehavior {
        let (mut behavior,mut transform) = pet;
        if behavior.direction == Vec2::ZERO { behavior.speed = 0; continue;}
        let dp:f32 = behavior.speed as f32*time.delta().as_secs_f32();
        let dir = behavior.direction.normalize_or_zero();
        transform.scale.x = dir.x.signum();
        transform.translation.x += dir.x*dp;
        transform.translation.y += dir.y*dp;
        let tx = transform.translation.x as i32 + 2i32.pow(layer::EXP_AREA-1);
        let ty = transform.translation.y as i32 + 2i32.pow(layer::EXP_AREA-1);
        let mut gx = tx>>layer::EXP_AREA;
        let mut gy = ty>>layer::EXP_AREA;
        behavior.gobalpos += IVec2::new(gx,gy);
        if gx != 0{gx *= 2i32.pow(layer::EXP_AREA);}
        if gy != 0{gy *= 2i32.pow(layer::EXP_AREA);}
        transform.translation -= Vec3::new(gx as f32, gy as f32,0.0);
    }
    
}
pub fn apply_setting(mut commands: Commands,sets:ResMut<Setting>) {
    commands.trigger(ESetting{target:sets.clone()});
    debug!("{:#?}",sets)

}