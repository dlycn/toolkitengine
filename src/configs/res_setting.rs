use bevy::window::*;

#[derive(Debug,Clone,Copy)]
pub struct Setinterface{
    pub ofx:i32,
    pub ofy:i32,
    pub width:u32,
    pub height:u32,
}

pub const INTERFACE: Setinterface = Setinterface {
    ofx: 150,
    ofy: 150,
    width: 1280,
    height: 720,
};

#[derive(Debug,Clone)]
pub struct Setinfo {
    pub title: String,
    pub presentmode: PresentMode,
    pub windowmode: WindowMode
}

impl Default for Setinfo {
    fn default() -> Self {Setinfo {
    title: "Toolkit Engine".to_string(),
    presentmode: PresentMode::AutoNoVsync,
    windowmode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
}
}}