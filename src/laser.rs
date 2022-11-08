use bevy::prelude::*;

use crate::{board::Position, palette, player::Player};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(laser_movement);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_RED,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.4, 0.0),
            ..default()
        })
        .insert(Position::from_xy(0, 2))
        .insert(Laser::new(Axis::Horizontal));
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_RED,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.4, 0.0),
            ..default()
        })
        .insert(Position::from_xy(0, -2))
        .insert(Laser::new(Axis::Horizontal));
}

#[derive(Component)]
struct Laser {
    pub movement: Axis,
}

impl Laser {
    pub fn new(movement: Axis) -> Self {
        Self { movement }
    }
}

enum Axis {
    Horizontal,
}

fn laser_movement(
    mut laser_query: Query<(&mut Position, &Laser), Without<Player>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_position = player_query.single().vec;
    for (mut laser_position, laser) in &mut laser_query {
        match laser.movement {
            Axis::Horizontal => laser_position.vec.x = player_position.x,
        }
    }
}
