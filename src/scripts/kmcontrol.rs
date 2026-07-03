use bevy::prelude::*;
use super::super::components::*;
use super::super::configs::res_sync;

pub fn control(
    mut query:Query<&mut Transform,With<Cselect>>,
    control:ResMut<res_sync::RControl>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>){
let fspeed = control.speed*time.delta_secs();
if let Ok(mut transform) = query.single_mut() {
     if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += fspeed;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= fspeed;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= fspeed;
    }
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += fspeed;
    }
}
}

