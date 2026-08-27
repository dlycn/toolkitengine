mod pet; // 私有引入
mod test;
mod ui;
mod map;
pub mod e2s {
    pub mod pet {
        pub use crate::entities::pet::*;
    }
    pub mod map {
        pub use crate::entities::map::*;
    }
    pub mod ui {
        pub use crate::entities::ui::*;
    }
}

