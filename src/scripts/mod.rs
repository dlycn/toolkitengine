use bevy::prelude::*;
use super::configs::*;
use super::components::*;
use super::events::*;
use crate::configs::res_hello::*;
use crate::events::setwindows::*;

pub mod kmcontrol;


pub fn system_ini(mut commands: Commands) {
    commands.spawn((
        Camera2d,Camera::default(),Cselect));
}

pub fn add_people(mut commands: Commands) {
    commands.spawn((Cperson, Cname("Elaina Proctor".to_string())));
    commands.spawn((Cperson, Cname("Renzo Hume".to_string())));
    commands.spawn((Cperson, Cname("Zayna Nieves".to_string())));
}

pub fn greet_people(mut commands: Commands,time: Res<Time>, mut timer: ResMut<RGreetTimer>, query: Query<&Cname, With<Cperson>>) {
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
    debug!("{:?}",sets)

}