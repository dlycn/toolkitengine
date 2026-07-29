use bevy::prelude::*;
use super::configs::*;
use super::components::*;
use super::events::*;
use crate::configs::res_hello::*;
use crate::events::setwindows::*;

pub mod kmcontrol;

pub fn greet_people(mut commands: Commands,time: Res<Time>, mut timer: ResMut<RGreetTimer>, query: Query<&Cpet>) {
    if timer.tick(time.delta()){
        commands.trigger(EUpdatePet {target:"Elaina Proctor".to_string(),parameter:"Elaina Hume".to_string()});
    }
    if timer.0.just_finished() {
        for pet in &query {
            println!("hello {}!", pet.name);
        }
    }
}

pub fn apply_setting(mut commands: Commands,sets:ResMut<Setting>) {
    commands.trigger(ESetting{target:sets.clone()});
    debug!("{:#?}",sets)

}