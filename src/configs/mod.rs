//use std::fs;
//use std::sync::OnceLock;
//use serde::{Deserialize,Serialize};
//
//const CONFIG_PATH:&str = r##"src/setting.toml"##;

pub mod res_hello;
pub mod res_sync;
pub mod res_ui;

mod res_setting;

use bevy::prelude::*;
use res_setting::*;

#[derive(Debug, Clone)]
pub struct SetInit {
    pub log: Setlog,
}

impl Default for SetInit {
    fn default() -> Self {
        SetInit {
            log: Setlog::default(),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct Setting {
    pub interface: Setinterface,
    pub info: Setinfo,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            interface: Setinterface::default(),
            info: Setinfo::default(),
        }
    }
}
