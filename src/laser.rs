use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{board::Position, palette, player::Player, AppState};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Alive)
                .with_system(movement)
                .with_system(attack)
                .into(),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let top = commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_RED,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..default()
        })
        .id();
    let bottom = commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_RED,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -2.0),
            ..default()
        })
        .id();
    let ray = commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 4.0))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_RED,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .id();
    commands
        .spawn_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0.0, 0.5, 0.0,
        )))
        .insert_bundle(VisibilityBundle::default())
        .insert(Position::from_xy(0, 0))
        .insert(Laser::new(Axis::Vertical, ray, 2.0, 0.2))
        .push_children(&[top, bottom, ray]);
}

#[derive(Component)]
struct Laser {
    pub axis: Axis,
    pub ray: Entity,
    pub timer: Timer,
    duration: Duration,
}

impl Laser {
    pub fn new(axis: Axis, ray: Entity, interval: f32, duration: f32) -> Self {
        Self {
            axis,
            ray,
            timer: Timer::from_seconds(interval, true),
            duration: Duration::from_secs_f32(duration),
        }
    }

    pub fn shooting(&self) -> bool {
        (self.timer.duration() - self.timer.elapsed()) < self.duration
    }
}

enum Axis {
    Vertical,
}

fn movement(
    mut laser_query: Query<(&mut Position, &Laser), Without<Player>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_position = player_query.single().vec;
    for (mut laser_position, laser) in &mut laser_query {
        match laser.axis {
            Axis::Vertical => laser_position.vec.x = player_position.x,
        }
    }
}

fn attack(
    mut laser_query: Query<(&Position, &mut Laser), Without<Player>>,
    mut player_query: Query<&Position, With<Player>>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let player_position = player_query.single_mut();
    for (laser_position, mut laser) in &mut laser_query {
        laser.timer.tick(time.delta());
        let mut visibility = visibility_query.get_mut(laser.ray).unwrap();
        visibility.is_visible = laser.shooting();
        if laser.shooting() && laser_position.vec == player_position.vec {
            commands.insert_resource(NextState(AppState::Dead));
        }
    }
}
