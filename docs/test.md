//src\plugins\mod.rs
pub mod glres;
use glres::*;
use super::scripts::*;
use super::configs::*;
use super::events::*;
额，这就直接使用二级依赖和模块公开了，显得怎么很厉害。
//src\scripts\mod.rs
use bevy::prelude::*;
use super::plugins::glres::*;
use super::components::*;
use super::events::*;
虽然是早期结构，但是总显得依赖链很大的感觉。