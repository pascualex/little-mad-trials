use std::time::Duration;

use bevy::prelude::*;

use crate::background::Countdown;

pub struct PhasesPlugin;

impl Plugin for PhasesPlugin {
    fn build(&self, _: &mut App) {}
}

#[derive(Component)]
pub struct Phases<T: Default + Send + Sync + 'static> {
    pub vec: Vec<Phase<T>>,
    pub start: Duration,
}

impl<T: Default + Clone + Copy + Send + Sync> Phases<T> {
    pub fn new(vec: Vec<Phase<T>>) -> Self {
        Self {
            vec,
            start: Duration::ZERO,
        }
    }

    pub fn mode(&self) -> T {
        match self.vec.first() {
            Some(phase) => phase.mode,
            None => T::default(),
        }
    }
}

pub struct Phase<T> {
    pub mode: T,
    pub duration: Duration,
}

impl<T> Phase<T> {
    pub fn new(mode: T, seconds: f32) -> Self {
        Self {
            mode,
            duration: Duration::from_secs_f32(seconds),
        }
    }
}

pub fn phases<T: Default + Send + Sync>(
    mut query: Query<&mut Phases<T>>,
    countdown: Res<Countdown>,
) {
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
