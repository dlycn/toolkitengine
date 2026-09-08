
use crate::events::pets::*;

use super::entities::e2s;
use super::configs::{res, SetInit, Setting, material};
use super::events::*;
use super::scripts::*;

use bevy::log::LogPlugin;

use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use bevy::winit::*;
use bevy::diagnostic::*;
use bevy_basisu_loader::BasisuLoaderPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(e_updatepet)
        .insert_resource(res::hello::RGreetTimer::default())
        .insert_resource(res::layer::Ruilayer::default())
        .insert_resource(res::pet::SelectedPet::default())
        .insert_resource(res::pet::CreatedPet::default())
        .add_observer(e_select_pet)    
        .add_observer(e_create_pet)    
        .add_systems(Startup, (
            e2s::ui::world::build.spawn(),
            e2s::map::setup.before(res_preload),
            e2s::pet::setup.after(res_preload),
            ))
        .add_systems(Startup, e2s::pet::pet_select.after(e2s::pet::setup))
        .add_systems(Update, (
            pet_behavior,
            greet_people,
            animate_sprite.before(pet_behavior)))
        .add_systems(PostUpdate, kmcontrol::petcontrol);
    }
}

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_observer(e_setlod)
        .add_systems(Startup, (
            (res_insert,res_preload,e2s::ui::system::build.spawn()).chain(),
            (apply::font,apply::shaders::global,apply::shaders::area)
        ).chain())
        
        .add_systems(PostUpdate,kmcontrol::syscontrol);
    }
}

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        let setinit:SetInit = SetInit::default();
        debug_once!("{:#?}",setinit);
        let mut mainwindow = Window::default();
        if setinit.mode.desktoptoys{
            mainwindow.transparent = true;      // 关键：背景透明
            mainwindow.decorations = false;     // 无边框
        }
        app.add_plugins(DefaultPlugins.set(LogPlugin{
            level:setinit.log.level,
            filter:setinit.log.filter,..default()},).set(WindowPlugin{
                primary_window: Some(mainwindow),
            ..default()
            }).set(AssetPlugin{
                ..default()
            })
        );
        app.add_observer(setwindows::e_setting);
        app.insert_resource(WinitSettings::game());
        app.insert_resource(Setting::default());
        app.add_plugins(BasisuLoaderPlugin);
        app.add_systems(Startup, apply_setting);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(FrameTimeDiagnosticsPlugin::default())  
        .add_plugins(LogDiagnosticsPlugin::default());
    }
}

pub struct RanderPlugin;

impl Plugin for RanderPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(UiMaterialPlugin::<material::HpMaterial>::default(),)
        .add_plugins(UiMaterialPlugin::<material::MapMaterial>::default(),)
        .add_plugins(UiMaterialPlugin::<material::ExpMaterial>::default(),)
        .add_plugins(UiMaterialPlugin::<material::StateMaterial>::default(),);
        app.add_plugins(Material2dPlugin::<material::AreaMaterial>::default(),);

    }
}
