use bevy::prelude::*;
use std::time::Duration;
#[derive(Resource)]
pub struct RGreetTimer(pub Timer, pub Option<Timer>);

impl RGreetTimer {
    pub fn tick(&mut self, delta: Duration) -> bool {
        self.0.tick(delta);
        let mut out:bool = false;
        if let Some(timer) = &mut self.1 {
            if timer.just_finished() {
                self.1 = None;
                out = true;
            } else {
                timer.tick(delta);
            }
        }
        out
    }
}

impl Default for RGreetTimer {
    fn default() -> Self {
        RGreetTimer(
            Timer::from_seconds(2.0, TimerMode::Repeating),
            Some(Timer::from_seconds(5.0, TimerMode::Once)),
        )
    }
}
