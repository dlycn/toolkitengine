use bevy::window::*;
use tracing::Level;

#[derive(Debug,Clone)]
pub struct Setlog{
    pub filter:String,
    pub level:Level,
}

impl Default for Setlog {
    fn default() -> Self {Setlog { 
        filter: "wgpu=error,naga=error,bevy_render=info,bevy_ecs=warn,bevy=info,toolkitengine=debug".to_string(), 
        level: Level::INFO }
}}


#[derive(Debug,Clone)]
pub struct Setinterface{
    pub ofx:i32,
    pub ofy:i32,
    pub width:u32,
    pub height:u32,
}

impl Default for Setinterface {
    fn default() -> Self {Setinterface {
    ofx: 150,
    ofy: 150,
    width: 1280,
    height: 720,}
}}

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