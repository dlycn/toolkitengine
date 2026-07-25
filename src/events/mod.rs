use bevy::prelude::*;
use crate::components::*; 

pub mod setwindows;

#[derive(Event)]
pub struct EUpdatepeople{
    pub target: Cname,
    pub parameter: String}

pub fn e_updatepeople(event: On<EUpdatepeople>,query: Query<&mut Cname, With<Cpet>>) {
    for mut name in query{
        if name.0 == event.target.0{name.0 = event.parameter.clone();println!("{} change to {}",&event.target.0,&event.parameter)}
    }}

#[derive(Event)]
pub struct ESetLod{
    pub parameter: u16}

pub fn e_setlod(event: On<ESetLod>,query: Query<&mut Visibility, With<Cpet>>) {
    let visopt = match event.parameter {
        0 => Visibility::Inherited,
        1 => Visibility::Hidden,
        _ => Visibility::Visible,
    };
    for mut visible in query{
        *visible = visopt;
    }}
