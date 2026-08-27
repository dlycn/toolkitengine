use bevy::prelude::*;
use crate::configs::{res,material};
use crate::events::ESetLod;
use res::global::RHandle;
pub fn font_apply(initres: Res<RHandle>,mut query_font: Query<&mut TextFont,With<TextFont>>) {
    if initres.font.is_none() {return;}
    let of = initres.font.as_ref().unwrap();
    let fs = FontSource::Handle(of.clone());
    for mut txtf in &mut query_font {
        txtf.font = fs.clone();
    }
}

pub fn global_shaders_apply(initres: Res<RHandle>,
    mut query_map_material: Single<&mut MaterialNode<material::MapMaterial>>,
    mut query_hp_material: Single<&mut MaterialNode<material::HpMaterial>>,
) {
    debug!("minimap and hp shader apply");
}

pub fn area_shaders_apply(
    mut commands: Commands,
    mut area_material: ResMut<Assets<material::AreaMaterial>>,
    mut query_material: Query<&mut MeshMaterial2d<material::AreaMaterial>>) {
    for mut mat in &mut query_material {
        mat.0 = area_material.add(material::AreaMaterial{color:LinearRgba::WHITE});
        debug!("area shader apply");
    }
    
    commands.trigger(ESetLod{parameter:0,center:IVec2::ZERO});

}