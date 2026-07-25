use bevy::prelude::*;
use super::configs::*;
use super::components::*;
use super::events::*;
use crate::configs::res_hello::*;
use crate::events::setwindows::*;

pub mod kmcontrol;


pub fn system_ini(mut commands: Commands) {
    commands.spawn((
        Camera2d,Camera::default(),Transform{..Default::default()},Cselect));
}

pub fn add_people(mut commands: Commands,asset_server: Res<AssetServer>) {
    commands.spawn((Cpet, Cname("Elaina Proctor".to_string()),
    Sprite{image: asset_server.load("imgs/Tachies/1.png"),
    custom_size: Some(Vec2::splat(64f32)),..default()}));
    commands.spawn((Cpet, Cname("Renzo Hume".to_string())));
    commands.spawn((Cpet, Cname("Zayna Nieves".to_string())));
}

pub fn greet_people(mut commands: Commands,time: Res<Time>, mut timer: ResMut<RGreetTimer>, query: Query<&Cname, With<Cpet>>) {
    if timer.tick(time.delta()){
        commands.trigger(EUpdatepeople {target:Cname("Elaina Proctor".to_string()),parameter:"Elaina Hume".to_string()});
    }
    if timer.0.just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn apply_setting(mut commands: Commands,sets:ResMut<Setting>) {
    commands.trigger(ESetting{target:sets.clone()});
    debug_once!("{:#?}",sets)

}