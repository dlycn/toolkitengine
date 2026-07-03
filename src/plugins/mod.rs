
use super::configs::*;
use super::events::*;
use super::scripts::*;

use res_hello::*;
use res_sync::*;

use bevy::prelude::*;
use bevy::winit::*;
use bevy::diagnostic::*;

pub struct HelloPlugin;
pub struct SystemPlugin;
pub struct InitPlugin;
pub struct DebugPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RGreetTimer::default());
        app.add_observer(e_updatepeople);
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
    }
}

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(RControl::default())
        .add_systems(Startup, (system_ini,test_mesh).chain())
        .add_systems(Update, gorotate)
        .add_systems(PostUpdate,kmcontrol::control);
    }
}

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_observer(setwindows::e_setting);
        app.insert_resource(WinitSettings::continuous());
        app.insert_resource(Setting::default());
        app.add_systems(Startup, apply_setting);
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(FrameTimeDiagnosticsPlugin::default())  
        .add_plugins(LogDiagnosticsPlugin::default());
    }
}