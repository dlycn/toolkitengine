mod configs;
mod plugins;
mod scripts;
mod components;
mod events;


use bevy::prelude::*;
use plugins::*;


fn main() {

    App::new()
        .add_plugins(InitPlugin)
        .add_plugins(HelloPlugin)
        .add_plugins(SystemPlugin)
        .add_plugins(DebugPlugin)
        .run();

}
