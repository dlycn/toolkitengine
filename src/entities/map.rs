use bevy::prelude::*;
use crate::components::ui;
use crate::configs::material::AreaMaterial;
use crate::configs::res::layer;
fn area(size:f32)->impl Scene{
    bsn!{
        ui::CUIarea
        Mesh2d(asset_value(Rectangle::new(size, size)))
        MeshMaterial2d::<AreaMaterial>::default()
        Transform::from_translation(Vec3::new(0.0, 0.0, layer::OFS_MAP))
    }
}

pub fn setup(mut commands:Commands,layer:Res<layer::Ruilayer>){
    let mut num = layer.map_areas.pow(2);
    loop {
        commands.spawn_scene(area(layer.map_sides as f32));
        num -= 1;
        if num == 0{
            break;
        }
    }
}