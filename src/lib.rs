mod board;
mod laser;
mod palette;
mod player;

use bevy::prelude::*;

use crate::{board::BoardPlugin, laser::LaserPlugin, player::PlayerPlugin};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BoardPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(LaserPlugin)
            .add_startup_system(setup);
    }
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
