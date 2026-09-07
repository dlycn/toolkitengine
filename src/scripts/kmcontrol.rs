use crate::configs::res::layer::LOD_IDX;
use crate::configs::res;
use crate::components::ui;
use crate::events::ESetLod;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::ops;
use bevy::prelude::*;

use super::super::components::*;
use super::super::configs::res_sync;

pub fn syscontrol(
    mut commands: Commands,
    mut cameraquery: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    layer: Res<res::layer::Ruilayer>,    
    cameracontrol: ResMut<res::sync::RControl>,
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
            debug_once!(
                "{},{},{}",
                transform.translation,
                projection2d.scale,
                scroll.delta.y
            );
            let map_sides = layer.map_sides as f32;
            let x = (transform.translation.x/map_sides as f32).round() as i32;
            let y = (transform.translation.y/map_sides as f32).round() as i32;

            let pre = projection2d.scale;

            let mut l = layer.lod_level as u8;//lod_level
            let mut o =0;//scroll delta
            if scroll.delta.y != 0. {
                projection2d.scale *= ops::powf(fscale, control.rate * scroll.delta.y);
            

            //scroll delta, view_area 0:不变, -1:变小, 1:变大
            o = scroll.delta.y.signum() as i8;
            
            //extra lod level,boolean
            let e = (o<0 && (layer.lod_level<LOD_IDX.len() as u8 -1)) as i8;

            l += e as u8;

            }
            let mut  cur = projection2d.scale;
            cur = f32::clamp(cur, control.minfactor, control.maxfactor);

            let ld = LOD_IDX[l as usize];
            let c1 = (cur-ld as f32)*(pre-ld as f32) < 0.;
            let c2 = layer.map_center != IVec2::new(x, y);

            if c1 || c2{
                let lv = layer.lod_level as i8 - o;
                let mut event = ESetLod{parameter:layer.lod_level,center:layer.map_center};
                if c1{event.parameter = (lv.max(0).min(LOD_IDX.len() as i8 -1)) as u8;}
                if c2{event.center = IVec2::new(x, y);}
                debug!("lod:{}; cur:{};pre:{},c1:{},c2:{}",event.parameter,cur,pre,c1,c2);
                commands.trigger(event);
            }
            
            if !input.any_pressed([
                KeyCode::ArrowUp,
                KeyCode::ArrowDown,
                KeyCode::ArrowLeft,
                KeyCode::ArrowRight,
            ]) && scroll.delta == Vec2::ZERO
            {
                return;
            }
            projection2d.scale = cur;

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
    mut camera_query: Query<
        (&Camera, &GlobalTransform, &mut Transform),
        (With<Cselect>, Without<Cpet>),
    >,
    grow_query: Query<&Children,With<ui::CUIinfo>>,
    mut info_query: Query<&mut Text>,
    mut petselect: ResMut<res::pet::SelectedPet>,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
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
        if keyinput {
            if let Ok((mut behavior, transform)) = petquery.get_mut(petselect.0.unwrap()) {
                transformca.translation = transform.translation;
                let petpos = transform.translation;
                for children in grow_query.iter() {  
                    let mut iter = info_query.iter_many_mut(children.iter());
                    while let Some(mut info) = iter.fetch_next() {
                        info.0 = format!("i:{}\nj:{}\nx:{:>5.0}\ny:{:>5.0}", behavior.gobalpos.x, behavior.gobalpos.y, petpos.x, petpos.y);
                    }
                }
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