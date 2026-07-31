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

pub fn pet_behavior(mut commands: Commands,mut query: Query<Entity,With<Cpet>>,mut querybehavior:Query<(&mut Cbehavior,&mut Transform)>,time: Res<Time>) {
    let pets = query.iter_mut().collect::<Vec<_>>();
    if pets.len() == 0 {return;};
    for pet in pets {
        if let Ok((mut behavior,mut transform)) = querybehavior.get_mut(pet){
            let dp:f32 = behavior.speed as f32*time.delta().as_secs_f32();
            transform.translation += behavior.direction.normalize_or_zero().extend(0.0);
            transform.translation.x += dp;
            transform.translation.y += dp;
        }
    }
    
}

pub fn apply_setting(mut commands: Commands,sets:ResMut<Setting>) {
    commands.trigger(ESetting{target:sets.clone()});
    debug!("{:#?}",sets)

}