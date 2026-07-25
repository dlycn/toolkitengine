use crate::events::ESetLod;
use bevy::math::ops::powf;
use bevy::prelude::*;

use super::super::components::*;
use super::super::configs::res_sync;

pub fn control(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Projection), (With<Cselect>, With<Camera2d>)>,
    control: ResMut<res_sync::RControl>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if !input.any_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::Comma,
        KeyCode::Period,
    ]) {
        return;
    }

    let fspeed = control.speed * time.delta_secs();
    let fscale = control.scale;
    if let Ok((mut transform, mut projection)) = query.single_mut() {
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

        if let Projection::Orthographic(projection2d) = &mut *projection {
            debug!("{},{}", transform.translation, projection2d.scale);
            let pre = projection2d.scale;
            if input.pressed(KeyCode::Comma) {
                projection2d.scale *= powf(fscale, time.delta_secs());
            }

            if input.pressed(KeyCode::Period) {
                projection2d.scale *= powf(fscale, -time.delta_secs());
            }
            let cur = projection2d.scale;
            match (cur < control.lodsprite, pre > control.lodsprite) {
                (true, true) => commands.trigger(ESetLod { parameter: 0 }),
                (false, false) => commands.trigger(ESetLod { parameter: 1 }),
                _ => (),
            }
            projection2d.scale = f32::max(cur, control.minfactor).min(control.maxfactor);
        }
    }
}
