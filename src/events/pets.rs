use bevy::prelude::*;
use bevy::math::bounding::BoundingCircle;
use crate::components::*;
use crate::configs::res_pet::*;

#[derive(EntityEvent)]
pub struct ESelectPet{
    pub entity: Entity,}

pub fn e_select_pet(
    event: On<ESelectPet>,
    mut respet: ResMut<SelectedPet>,
) {
    respet.0 = Some(event.entity);
}