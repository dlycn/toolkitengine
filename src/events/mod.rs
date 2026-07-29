use bevy::prelude::*;
use crate::components::*; 

pub mod setwindows;

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
