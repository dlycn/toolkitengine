use bevy::prelude::*;
use bevy::math::Quat;
use super::configs::*;
use super::components::*;
use super::events::*;
use crate::configs::res_hello::*;
use crate::events::setwindows::*;

pub mod kmcontrol;


pub fn system_ini(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Cselect));
    commands.spawn((
        Camera2d,
        Camera {order: 1,..default()}));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
));
}

pub fn gorotate(mut query: Query<&mut Transform, With<Ctag>>,time: Res<Time>) {
    if let Ok(mut transform) = query.single_mut() {
    let axis = Vec3::new(0.5,0.5,1.0).normalize_or_zero();
    let angle = 3.14;
    let rotation = Quat::from_axis_angle(axis, angle*time.delta().as_secs_f32());
    transform.rotate(rotation);
    } else {
        eprintln!("Expected exactly one entity with Transform and Ctag, but found a different number.");
    }
}


pub fn test_mesh(mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial{base_color:Color::WHITE,..default()})),Ctag));
}


pub fn add_people(mut commands: Commands) {
    commands.spawn((Cperson, Cname("Elaina Proctor".to_string())));
    commands.spawn((Cperson, Cname("Renzo Hume".to_string())));
    commands.spawn((Cperson, Cname("Zayna Nieves".to_string())));
}

pub fn greet_people(mut commands: Commands,time: Res<Time>, mut timer: ResMut<RGreetTimer>, query: Query<&Cname, With<Cperson>>) {
    if timer.tick(time.delta()){
        commands.trigger(EUpdatepeople {target:Cname("Elaina Proctor".to_string()),parameter:"Elaina Hume".to_string()});
    }
    if timer.0.just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn apply_setting(mut commands: Commands,sets:ResMut<Setting>) {
    commands.trigger(ESetting{target:sets.clone()});
    println!("{:?}",sets)

}