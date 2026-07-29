use crate::events::ESetLod;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::ops::powf;
use bevy::prelude::*;

use super::super::components::*;
use super::super::configs::res_sync;

pub fn control(
    mut commands: Commands,
    mut cameraquery: Query<(&mut Transform, &mut Projection), (With<Cselect>, With<Camera2d>)>,
    cameracontrol: ResMut<res_sync::RControl>,
    input: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    if !input.any_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight
    ]) && scroll.delta == Vec2::ZERO {
        return;
    }

    let fspeed = cameracontrol.speed * time.delta_secs();
    let fscale = cameracontrol.scale;
    if let Ok((mut transform, mut projection)) = cameraquery.single_mut() {
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
        
    let control = cameracontrol;
    
        if let Projection::Orthographic(projection2d) = &mut *projection {
            debug!("{},{},{}", transform.translation, projection2d.scale,scroll.delta.y);
            let pre = projection2d.scale;

            if scroll.delta.y!=0. {
                projection2d.scale *= powf(fscale, control.rate*scroll.delta.y);
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
