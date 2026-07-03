use bevy::prelude::*;
use crate::components::*; 

pub mod setwindows;

#[derive(Event)]
pub struct EUpdatepeople{
    pub target: Cname,
    pub parameter: String}

pub fn e_updatepeople(event: On<EUpdatepeople>,query: Query<&mut Cname, With<Cperson>>) {
    for mut name in query{
        if name.0 == event.target.0{name.0 = event.parameter.clone();println!("{} change to {}",&event.target.0,&event.parameter)}
    }}

