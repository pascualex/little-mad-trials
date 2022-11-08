mod movement;
mod palette;

use bevy::prelude::*;
use movement::{Board, MovementPlugin, Player, Position};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MovementPlugin)
            .add_startup_system(setup)
            .add_system(turret_movement);
    }
}

fn setup(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.8))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_BLUE,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.4, 0.0),
            ..default()
        })
        .insert(Position::from_xy(0, 0))
        .insert(Player);
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
        .insert(Turret::new(Axis::Horizontal));
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
        .insert(Turret::new(Axis::Horizontal));
    for i in -1..=1 {
        for j in -1..=1 {
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.92, 4.0, 0.92))),
                material: materials.add(StandardMaterial {
                    base_color: palette::LIGHT_WHITE,
                    metallic: 0.1,
                    perceptual_roughness: 0.7,
                    reflectance: 0.3,
                    ..default()
                }),
                transform: Transform::from_xyz(j as f32, -2.0, -i as f32),
                ..default()
            });
            board.tiles.insert(IVec2::new(j, i));
        }
    }
}

#[derive(Component)]
struct Turret {
    pub movement: Axis,
}

impl Turret {
    pub fn new(movement: Axis) -> Self {
        Self { movement }
    }
}

enum Axis {
    Horizontal,
}

fn turret_movement(
    mut turret_query: Query<(&mut Position, &Turret), Without<Player>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_position = player_query.single().vec;
    for (mut turret_position, turret) in &mut turret_query {
        match turret.movement {
            Axis::Horizontal => turret_position.vec.x = player_position.x,
        }
    }
}
