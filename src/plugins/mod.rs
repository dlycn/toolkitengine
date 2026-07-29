
use super::entities::ui;
use super::configs::*;
use super::events::*;
use super::scripts::*;

use bevy::log::LogPlugin;
use res_hello::*;
use res_sync::*;

use bevy::prelude::*;
use bevy::winit::*;
use bevy::diagnostic::*;

pub struct WorldPlugin;
pub struct SystemPlugin;
pub struct InitPlugin;
pub struct DebugPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RGreetTimer::default());
        app.add_observer(e_updatepet)
        .add_systems(Startup, ui::world::build.spawn())
        .add_systems(Update, greet_people);
    }
}

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(RControl::default())
        .add_observer(e_setlod)
        .add_systems(Startup, ui::system::build.spawn())
        .add_systems(PostUpdate,kmcontrol::control);
    }
}



impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        let setinit:SetInit = SetInit::default();
        debug_once!("{:#?}",setinit);
        app.add_plugins(DefaultPlugins.set(LogPlugin{
            level:setinit.log.level,
            filter:setinit.log.filter,..default()}));
        app.add_observer(setwindows::e_setting);
        app.insert_resource(WinitSettings::game());
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