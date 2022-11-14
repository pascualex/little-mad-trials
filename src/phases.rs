use std::time::Duration;

use bevy::prelude::*;

use crate::background::Countdown;

pub struct PhasesPlugin;

impl Plugin for PhasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(phases);
    }
}

#[derive(Component)]
pub struct Phases {
    pub vec: Vec<Phase>,
    pub start: Duration,
}

impl Phases {
    pub fn new(vec: Vec<Phase>) -> Self {
        Self {
            vec,
            start: Duration::ZERO,
        }
    }

    pub fn mode(&self) -> Mode {
        match self.vec.first() {
            Some(phase) => phase.mode,
            None => Mode::Ready,
        }
    }
}

pub struct Phase {
    pub mode: Mode,
    pub duration: Duration,
}

impl Phase {
    pub fn new(mode: Mode, seconds: f32) -> Self {
        Self {
            mode,
            duration: Duration::from_secs_f32(seconds),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Mode {
    Ready,
    Charging,
    Shooting,
}

pub fn phases(mut query: Query<&mut Phases>, countdown: Res<Countdown>) {
    for mut laser in &mut query {
        let duration = match laser.vec.first() {
            Some(phase) => phase.duration,
            None => continue,
        };
        if countdown.timer.elapsed() >= laser.start + duration {
            laser.start += duration;
            laser.vec.remove(0);
        }
    }
}
