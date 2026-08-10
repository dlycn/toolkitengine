
use crate::events::pets::e_select_pet;

use super::entities::*;
use super::configs::{res, SetInit, Setting};
use super::events::*;
use super::scripts::*;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::*;
use bevy::diagnostic::*;

pub struct WorldPlugin;
pub struct SystemPlugin;
pub struct InitPlugin;
pub struct DebugPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(res::hello::RGreetTimer::default());
        app.insert_resource(res::ui::Ruilayer::default());
        app.insert_resource(res::pet::SelectedPet::default());
        app.add_observer(e_updatepet)
        .add_observer(e_select_pet)        
        .add_systems(Startup, (
            e2s_ui::world::build.spawn(),
            e2s_pet::setup))
        .add_systems(Update, (
            pet_behavior,
            greet_people,
            animate_sprite.before(pet_behavior)))
        .add_systems(PostUpdate, kmcontrol::petcontrol);
    }
}

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(res::sync::RControl::default())
        .add_observer(e_setlod)
        .add_systems(Startup, e2s_ui::system::build.spawn())
        .add_systems(PostUpdate,kmcontrol::syscontrol);
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