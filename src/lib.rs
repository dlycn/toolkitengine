mod configs;
mod plugins;
mod scripts;
mod entities;
mod components;
mod events;
use bevy::{asset::io::AssetSourceBuilder, prelude::*};
use plugins::{WorldPlugin, SystemPlugin,RanderPlugin};
use crate::{configs::res::setting::Setinfo, plugins::InitPlugin};
mod apis;
pub mod studio{
    pub use crate::plugins::{DebugPlugin};
    pub use crate::CorePlugin;
    pub use crate::apis::HungrySnakePlugin;
}


pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app        
        .add_plugins(WorldPlugin)
        .add_plugins(SystemPlugin)
        .add_plugins(RanderPlugin);
    }
}

pub trait Pre{
    fn param(info:&Setinfo) -> (String, Option<String>){
        let title = info.title.clone();
        let path = Some(format!("assets/modules/{}",title));
        (title, path)
    }
    fn info() -> Setinfo;
}

impl Pre for CorePlugin{
    fn param(info:&Setinfo) -> (String, Option<String>) {
        (info.title.clone(), None)
    }

    fn info() -> Setinfo{
        Setinfo::default()
    }
}

impl Pre for apis::HungrySnakePlugin {fn info() -> Setinfo{Setinfo{title:"HungrySnake".to_string(),..default()}}}

pub struct Launcher;
impl Launcher {
    pub fn on<T: Pre+Plugin>(stdplugin:T) -> App {
        let mut app = App::new();
        let iniplugin = InitPlugin;
        let (title, path) = T::param(&T::info());
        if let Some(path) = path{
        app.register_asset_source(title,
             AssetSourceBuilder::platform_default(path.as_str(), None));}
        app.add_plugins(iniplugin)
        .add_plugins(stdplugin);
        app
    }
}
