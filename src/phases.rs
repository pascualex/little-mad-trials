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
    pub progress: f32,
}

impl<T: Default + Clone + Copy + Send + Sync> Phases<T> {
    pub fn new(vec: Vec<Phase<T>>) -> Self {
        Self {
            vec,
            start: Duration::ZERO,
            progress: 0.0,
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

pub fn transition<T: Default + Send + Sync>(
    mut query: Query<&mut Phases<T>>,
    countdown: Res<Countdown>,
) {
    for mut phases in &mut query {
        let duration = match phases.vec.first() {
            Some(phase) => phase.duration,
            None => continue,
        };
        if countdown.timer.elapsed() >= phases.start + duration {
            phases.start += duration;
            phases.vec.remove(0);
        }
        let elapsed = countdown.timer.elapsed() - phases.start;
        phases.progress = match phases.vec.first() {
            Some(phase) => match phase.duration {
                Duration::ZERO => 1.0,
                _ => f32::min(elapsed.as_secs_f32() / phase.duration.as_secs_f32(), 1.0),
            },
            None => 1.0,
        };
    }
}
