use bevy::prelude::*;
use bevy::window::*;
use crate::configs::Setting;

#[derive(Event)]
pub struct ESetting{
    pub target: Setting}

pub fn e_setting(event:On<ESetting>,mut query:Query<&mut Window, With<PrimaryWindow>>){
    if let Ok(mut window) = query.single_mut() {
        let setting = &event.target;
        window.present_mode = setting.info.presentmode;
        window.title = setting.info.title.clone();
        window.mode = setting.info.windowmode;
        if window.mode == WindowMode::Windowed{
        window.position = WindowPosition::At(IVec2::new(
            setting.interface.ofx,
            setting.interface.ofy,
        ));
        window.resolution = WindowResolution::new(
            setting.interface.width,
            setting.interface.height,
        );}
    }
}