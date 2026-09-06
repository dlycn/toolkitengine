use bevy::prelude::*;
<<<<<<< HEAD
use crate::configs::res::pet::*;
=======
use bevy::math::bounding::BoundingCircle;
use crate::components::*;
use crate::configs::res_pet::*;
>>>>>>> parent of e94b3f5 (pass)

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
