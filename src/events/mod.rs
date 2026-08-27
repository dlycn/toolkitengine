use bevy::{prelude::*};
use crate::{components::*, 
    configs::res::layer}; 

pub mod setwindows;
pub mod pets;

#[derive(Event)]
pub struct EUpdatePet{
    pub target: String,
    pub parameter: String}

pub fn e_updatepet(event: On<EUpdatePet>,query: Query<&mut Cpet>) {
    for mut pet in query{
        if pet.name == event.target{pet.name = event.parameter.clone();println!("{} change to {}",&event.target,&event.parameter)}
    }}

#[derive(Event)]
pub struct ESetLod{
    pub parameter: u8,
    pub center: IVec2}

pub fn e_setlod(event: On<ESetLod>,mut res: ResMut<layer::Ruilayer>,
    query: Query<&mut Visibility, With<Cpet>>,
    mut mapquery: Query<&mut Transform,With<ui::CUIarea>>){
        res.lod_level = event.parameter;
        res.map_sides = layer::LOD_MAP[res.lod_level as usize]*layer::STD_MAP;
        res.map_center = event.center;
    let scale = Vec3::new(layer::LOD_MAP[res.lod_level as usize] as f32,
        layer::LOD_MAP[res.lod_level as usize] as f32,1.0);
    let base = res.map_sides as i32;
    let ext = (!res.map_areas & 1) as u32 * res.map_sides/2;
    let extra = (res.map_areas>>1) as i32;
    for (i, mut transform) in mapquery.iter_mut().enumerate(){
        let pos = res.map_prepos[i];
        let x:i32 = pos.x+res.map_center.x-extra;
        let y:i32 = pos.y+res.map_center.y-extra;
        transform.translation.x = (base as i32*x) as f32 + ext as f32;
        transform.translation.y = (base as i32*y) as f32 + ext as f32;
        transform.scale = scale;
    }


    let visopt = match res.lod_level>2 {
        false => Visibility::Inherited,
        true => Visibility::Hidden,
    };
    for mut visible in query{
        *visible = visopt;
    }}
