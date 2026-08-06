use crate::events::{ESetLod,pets::ESelectPet};
use crate::configs::res_pet::*;
use bevy::input::mouse::{AccumulatedMouseScroll};
use bevy::math::ops;
use bevy::prelude::*;

use super::super::components::*;
use super::super::configs::res_sync;

pub fn syscontrol(
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

    let fscale = cameracontrol.scale;
    if let Ok((mut transform, mut projection)) = cameraquery.single_mut() {
        
    let control = cameracontrol;
    
        if let Projection::Orthographic(projection2d) = &mut *projection {
            debug_once!("{},{},{}", transform.translation, projection2d.scale,scroll.delta.y);
            let pre = projection2d.scale;
            if scroll.delta.y!=0. {
                projection2d.scale *= ops::powf(fscale, control.rate*scroll.delta.y);
            }
            let cur = projection2d.scale;
            match (cur < control.lodsprite, pre > control.lodsprite) {
                (true, true) => commands.trigger(ESetLod { parameter: 0 }),
                (false, false) => commands.trigger(ESetLod { parameter: 1 }),
                _ => (),
            }
            projection2d.scale = f32::clamp(cur, control.minfactor, control.maxfactor);
        
            let fspeed = control.speed * projection2d.scale * time.delta_secs();

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
}

pub fn petcontrol(
    mut commands: Commands,
    mut petquery: Query<(&mut Cbehavior, &mut Transform), With<Cpet>>,
    camera_query: Single<(&Camera, &GlobalTransform),With<Cselect>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut petselect: ResMut<SelectedPet>,
    window: Single<&Window>,
    time: Res<Time>,
) {
    if let Some(cursor_position) = window.cursor_position(){
        let (camera, global_transform) = *camera_query;
        let worldpos = camera.viewport_to_world_2d(global_transform,cursor_position).expect("error: pos changed from viewport to the world");
        
        if mouse.just_pressed(MouseButton::Right) {
            petselect.0 = None;
        }
        if mouse.just_pressed(MouseButton::Left) {
            debug_once!("{:?}<->{:?}",cursor_position,worldpos);
        }

    }

    let keyinput = petselect.0.is_some();
    if keyinput {
        if !input.any_pressed([
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyA,
        KeyCode::KeyD
        ]){return;}
        if let Ok((mut behavior,_)) = petquery.get_mut(petselect.0.unwrap()){
            let mut dx = 0.0;
            let mut dy = 0.0;
            if input.pressed(KeyCode::KeyW) {
                dy += 1.0;
            }
            if input.pressed(KeyCode::KeyS) {
                dy -= 1.0;
            }
            if input.pressed(KeyCode::KeyA) {
                dx -= 1.0;
            }
            if input.pressed(KeyCode::KeyD) {
                dx += 1.0;
            }
            behavior.direction = Vec2::new(dx,dy);
            debug!("{:?}",behavior.direction);
        }
        return;
    }
}