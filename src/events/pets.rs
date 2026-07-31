use bevy::prelude::*;
use crate::components::*;

#[derive(EntityEvent)]
pub struct ESelectPet{
    pub entity: Entity,}

pub fn e_select_pet(event:On<ESelectPet>,query: Query<Entity, (With<Cpet>,With<Cselect>)>,mut commands: Commands){
    let olds:Vec<Entity> = query.iter().collect();
    let num = olds.len();
        match num {
            0 => debug!("select pet init"),
            1 => debug!("select pet"),
            _ => debug!("select pet error"),
        };
    if let Ok(mut pet) = commands.get_entity(event.entity) {
        pet.insert(Cselect);
        if num>0{
            olds.iter().for_each(|e|{commands.get_entity(*e).unwrap().remove::<Cselect>();});
        };
    }
}