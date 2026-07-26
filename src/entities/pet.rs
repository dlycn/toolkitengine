use bevy::prelude::*;
use crate::components::*;
pub fn new(asset_server: &AssetServer,name:&str,path:String) -> impl Bundle {
    (Cpet, 
    Cname(name.to_string()),
    Sprite{image: asset_server.load(path),
    custom_size: Some(Vec2::splat(64f32)),..default()})
}