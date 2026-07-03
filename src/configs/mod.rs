//use std::fs;
//use std::sync::OnceLock;
//use serde::{Deserialize,Serialize};
//
//const CONFIG_PATH:&str = r##"src/setting.toml"##;

pub mod res_hello;
pub mod res_sync;

mod res_setting;

use bevy::prelude::*;
use res_setting::*;

#[derive(Debug, Clone, Resource)]
pub struct Setting {
    pub interface: Setinterface,
    pub info: Setinfo,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            interface: INTERFACE,
            info: Setinfo::default(),
        }
    }
}
