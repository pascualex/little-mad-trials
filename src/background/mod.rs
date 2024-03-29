mod fog;
mod screen;

use std::{f32::consts::PI, time::Duration};

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{material_from_color, palette, AppState};

use self::{fog::FogPlugin, screen::ScreenPlugin};

const WALLS_COLOR: Color = palette::DARK_BLACK;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FogPlugin)
            .add_plugin(ScreenPlugin)
            .insert_resource(Countdown::new())
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(
                SystemSet::on_update(AppState::Setup)
                    .with_system(countdown)
                    .with_system(transition.after(countdown)),
            )
            .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(countdown)
                    .with_system(transition.after(countdown)),
            )
            .add_system_set(SystemSet::on_enter(AppState::Defeat).with_system(enter_defeat))
            .add_system_set(SystemSet::on_enter(AppState::Victory).with_system(enter_victory))
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(countdown))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system_set(
                SystemSet::on_update(AppState::Teardown)
                    .with_system(countdown)
                    .with_system(transition.after(countdown)),
            )
            .add_system_set(SystemSet::on_exit(AppState::Teardown).with_system(exit_teardown));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // main wall
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1000.0, 1000.0)))),
            material: materials.add(StandardMaterial {
                ..material_from_color(WALLS_COLOR)
            }),
            transform: Transform::from_xyz(0.0, 0.0, -25.0),
            ..default()
        },
        NotShadowCaster,
    ));
    // left wall
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1000.0, 1000.0)))),
            material: materials.add(material_from_color(WALLS_COLOR * 0.9)),
            transform: Transform::from_xyz(-18.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
    // rigth wall
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1000.0, 1000.0)))),
            material: materials.add(material_from_color(WALLS_COLOR * 0.9)),
            transform: Transform::from_xyz(18.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
    // deep floor
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1000.0, 1000.0)))),
            material: materials.add(material_from_color(WALLS_COLOR * 0.8)),
            transform: Transform::from_xyz(0.0, -15.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

fn enter_setup(mut countdown: ResMut<Countdown>) {
    countdown.reset(1.5, Some(AppState::Start));
}

fn enter_start(mut countdown: ResMut<Countdown>) {
    countdown.reset(20.0, Some(AppState::Victory));
}

fn enter_defeat(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let sound = asset_server.load("sounds/defeat.ogg");
    audio.play_with_settings(sound, PlaybackSettings::ONCE.with_volume(0.6));
}

fn enter_victory(
    mut countdown: ResMut<Countdown>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    countdown.reset(1.0, None);
    let sound = asset_server.load("sounds/victory.ogg");
    audio.play_with_settings(sound, PlaybackSettings::ONCE.with_volume(0.3));
}

fn enter_teardown(mut countdown: ResMut<Countdown>) {
    countdown.reset(1.5, Some(AppState::Setup));
}

fn exit_teardown(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let sound = asset_server.load("sounds/reset.ogg");
    audio.play_with_settings(sound, PlaybackSettings::ONCE.with_volume(0.15));
}

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
    pub transition: Option<AppState>,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            transition: None,
        }
    }

    pub fn reset(&mut self, seconds: f32, transition: Option<AppState>) {
        self.timer.set_duration(Duration::from_secs_f32(seconds));
        self.timer.reset();
        self.transition = transition;
    }
}

pub fn countdown(mut countdown: ResMut<Countdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
}

fn transition(countdown: Res<Countdown>, mut state: ResMut<State<AppState>>) {
    let Some(transition) = countdown.transition else {
        return;
    };
    if countdown.timer.finished() {
        state.overwrite_set(transition).unwrap();
    }
}
