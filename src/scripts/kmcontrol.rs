use crate::configs::res;
use crate::events::ESetLod;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::ops;
use bevy::prelude::*;

use super::super::components::*;

pub fn syscontrol(
    mut commands: Commands,
    mut cameraquery: Query<(&mut Transform, &mut Projection), (With<Cselect>, With<Camera2d>)>,
    cameracontrol: ResMut<res::sync::RControl>,
    input: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
) {
    let fscale = cameracontrol.scale;
    if let Ok((mut transform, mut projection)) = cameraquery.single_mut() {
        if !input.any_pressed([
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
        ]) && scroll.delta == Vec2::ZERO
        {
            return;
        }
        let control = cameracontrol;

        if let Projection::Orthographic(projection2d) = &mut *projection {
            debug_once!(
                "{},{},{}",
                transform.translation,
                projection2d.scale,
                scroll.delta.y
            );
            let pre = projection2d.scale;
            if scroll.delta.y != 0. {
                projection2d.scale *= ops::powf(fscale, control.rate * scroll.delta.y);
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
    mut petquery: Query<(&mut Cbehavior, &mut Transform), With<Cpet>>,
    mut camera_query: Query<
        (&Camera, &GlobalTransform, &mut Transform),
        (With<Cselect>, Without<Cpet>),
    >,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut petselect: ResMut<res::pet::SelectedPet>,
    window: Single<&Window>
) {
    let keyinput = petselect.0.is_some();
    let willnum = 3.0;
    if let Ok((camera, global_transform, mut transformca)) = camera_query.single_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            let worldpos = camera
                .viewport_to_world_2d(global_transform, cursor_position)
                .expect("error: pos changed from viewport to the world");

            if mouse.just_pressed(MouseButton::Left) {
                debug_once!("{:?}<->{:?}", cursor_position, worldpos);
            }
        }
        if let Ok((mut behavior, transform)) = petquery.get_mut(petselect.0.unwrap()) {
            transformca.translation = transform.translation;
            if keyinput {
                if !input.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD])
                    && !mouse.just_pressed(MouseButton::Right)
                {
                    return;
                }
                if mouse.just_pressed(MouseButton::Right) {
                    petselect.0 = None;
                    behavior.direction = Vec2::ZERO;
                    behavior.speed = 0;
                }
                if input.pressed(KeyCode::KeyW) {
                    behavior.direction.y += 1.0;
                }
                if input.pressed(KeyCode::KeyS) {
                    behavior.direction.y -= 1.0;
                }
                if input.pressed(KeyCode::KeyA) {
                    behavior.direction.x -= 1.0;
                }
                if input.pressed(KeyCode::KeyD) {
                    behavior.direction.x += 1.0;
                }
                behavior.speed = res::pet::PET_MAX_SPEED;
                behavior.direction.x = behavior.direction.x.clamp(-willnum, willnum);
                behavior.direction.y = behavior.direction.y.clamp(-willnum, willnum);
                debug!("{:?},{}", behavior.direction, behavior.speed);
            }

            return;
        }
    }
}
