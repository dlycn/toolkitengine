//use std::fs;
//use std::sync::OnceLock;
//use serde::{Deserialize,Serialize};
//
//const CONFIG_PATH:&str = r##"src/setting.toml"##;

mod res_global;
mod res_hello;
mod res_pet;
mod res_setting;
mod res_sync;
mod res_ui;

pub mod res{
    pub mod hello{pub use crate::configs::res_hello::*;}
    pub mod pet{pub use crate::configs::res_pet::*;}
    pub mod setting{pub use crate::configs::res_setting::*;}
    pub mod sync{pub use crate::configs::res_sync::*;}
    pub mod ui{pub use crate::configs::res_ui::*;}
}


use bevy::prelude::*;
use res::setting::*;

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
