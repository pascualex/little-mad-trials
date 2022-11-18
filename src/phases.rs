use std::time::Duration;

use bevy::prelude::*;

use crate::background::Countdown;

pub struct PhasesPlugin;

impl Plugin for PhasesPlugin {
    fn build(&self, _: &mut App) {}
}

#[derive(Component)]
pub struct Phases<T: Send + Sync + 'static> {
    pub vec: Vec<Phase<T>>,
    pub start: Duration,
    pub progress: f32,
    pub just_reset: bool,
    pub just_transitioned: bool,
}

impl<T: Clone + Copy + Send + Sync> Phases<T> {
    pub fn new(base: T) -> Self {
        Self {
            vec: vec![Phase::new(base, 0.0)],
            start: Duration::ZERO,
            progress: 0.0,
            just_reset: true,
            just_transitioned: true,
        }
    }

    pub fn mode(&self) -> T {
        match self.vec.first() {
            Some(phase) => phase.mode,
            None => unreachable!(),
        }
    }

    pub fn reset(&mut self, vec: Vec<Phase<T>>) {
        self.vec = vec;
        self.start = Duration::ZERO;
        self.progress = 0.0;
        self.just_reset = true;
        self.just_transitioned = true;
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

pub fn transition<T: Send + Sync>(mut query: Query<&mut Phases<T>>, countdown: Res<Countdown>) {
    for mut phases in &mut query {
        match phases.just_reset {
            true => phases.just_reset = false,
            false => phases.just_transitioned = false,
        }
        if phases.vec.len() <= 1 {
            continue;
        }
        let duration = phases.vec.first().unwrap().duration;
        if countdown.timer.elapsed() >= phases.start + duration || countdown.timer.finished() {
            phases.start += duration;
            phases.vec.remove(0);
            phases.just_transitioned = true;
        }
        let elapsed = match countdown.timer.elapsed() >= phases.start {
            true => countdown.timer.elapsed() - phases.start,
            false => Duration::ZERO,
        };
        let duration = phases.vec.first().unwrap().duration;
        phases.progress = match duration {
            Duration::ZERO => 1.0,
            _ => f32::min(elapsed.as_secs_f32() / duration.as_secs_f32(), 1.0),
        };
    }
}
