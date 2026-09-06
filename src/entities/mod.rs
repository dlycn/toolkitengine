<<<<<<< HEAD
mod pet; // 私有引入
mod test;
mod ui;
mod map;
mod global;
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

=======
pub mod pet;
pub mod ui;
pub mod test;
>>>>>>> parent of e94b3f5 (pass)
