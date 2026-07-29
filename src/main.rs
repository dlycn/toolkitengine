mod configs;
mod plugins;
mod scripts;
mod entities;
mod components;
mod events;


use bevy::prelude::*;
use plugins::*;


fn main() {

    App::new()
        .add_plugins(InitPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(SystemPlugin)
        .add_plugins(DebugPlugin)
        .run();

}
