use bevy::prelude::*;
use crate::components::*;
use crate::configs::res::pet::*;

#[derive(EntityEvent)]
pub struct ESelectPet{
    pub entity: Entity,}

pub fn e_select_pet(
    event: On<ESelectPet>,
    mut respet: ResMut<SelectedPet>,
) {
    respet.0 = Some(event.entity);
    debug!("{:?}",respet.0);
}
