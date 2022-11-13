mod background;
mod board;
mod laser;
mod palette;
mod player;

use background::BackgroundPlugin;
use bevy::prelude::*;

use self::{board::BoardPlugin, laser::LaserPlugin, player::PlayerPlugin};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Setup)
            .add_plugin(BackgroundPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(LaserPlugin)
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(instant_victory))
            .add_system_set(SystemSet::on_update(AppState::Defeat).with_system(restart))
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(restart))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum AppState {
    Setup,
    Game,
    Defeat,
    Victory,
    Teardown,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    let size = 11.0;
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ZERO)
            .looking_at(Vec3::new(-1.0, -3.0, -2.0), Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 32_000.0,
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -size,
                right: size,
                bottom: -size,
                top: size,
                near: -size,
                far: size,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn enter_setup(mut state: ResMut<State<AppState>>) {
    state.overwrite_set(AppState::Game).unwrap();
}

fn enter_teardown(mut state: ResMut<State<AppState>>) {
    state.overwrite_set(AppState::Setup).unwrap();
}

fn restart(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Space) {
        state.set(AppState::Teardown).unwrap();
        input.clear(); // avoids infinite loops until stageless
    }
}

fn instant_victory(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::V) {
        state.set(AppState::Victory).unwrap();
        input.clear(); // avoids infinite loops until stageless
    }
}
