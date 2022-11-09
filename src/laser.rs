use std::time::Duration;

use bevy::{ecs::system::EntityCommands, prelude::*};
use iyes_loopless::prelude::*;

use crate::{board::Position, palette, player::Player, AppState};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::Setup, enter_setup)
            .add_enter_system(AppState::Teardown, enter_teardown)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Game)
                    .with_system(movement)
                    .with_system(charge)
                    .with_system(attack)
                    .into(),
            );
    }
}

fn enter_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let normal = laser_model(
        commands.spawn(),
        palette::DARK_YELLOW,
        &mut meshes,
        &mut materials,
    );
    let charging = laser_model(
        commands.spawn(),
        palette::DARK_RED,
        &mut meshes,
        &mut materials,
    );
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
        .insert(Laser::new(
            Axis::Vertical,
            2.0,
            0.5,
            0.2,
            normal,
            charging,
            ray,
        ))
        .push_children(&[normal, charging, ray]);
}

fn laser_model(
    mut commands: EntityCommands,
    color: Color,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let top = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
        material: materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    };
    let bottom = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
        material: materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    };
    commands
        .insert_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .with_children(|builder| {
            builder.spawn_bundle(top);
            builder.spawn_bundle(bottom);
        })
        .id()
}

fn enter_teardown(query: Query<Entity, With<Laser>>, mut commands: Commands) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

#[derive(Component)]
struct Laser {
    pub axis: Axis,
    pub timer: Timer,
    charge_duration: Duration,
    attack_duration: Duration,
    pub normal: Entity,
    pub charging: Entity,
    pub ray: Entity,
}

impl Laser {
    pub fn new(
        axis: Axis,
        interval: f32,
        charge_duration: f32,
        attack_duration: f32,
        normal: Entity,
        charging: Entity,
        ray: Entity,
    ) -> Self {
        Self {
            axis,
            timer: Timer::from_seconds(interval, true),
            charge_duration: Duration::from_secs_f32(charge_duration),
            attack_duration: Duration::from_secs_f32(attack_duration),
            normal,
            charging,
            ray,
        }
    }

    pub fn charging(&self) -> bool {
        self.remaining() <= self.charge_duration + self.attack_duration
    }

    pub fn shooting(&self) -> bool {
        self.remaining() <= self.attack_duration
    }

    fn remaining(&self) -> Duration {
        self.timer.duration() - self.timer.elapsed()
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
        if !laser.charging() {
            match laser.axis {
                Axis::Vertical => laser_position.vec.x = player_position.x,
            }
        }
    }
}

fn charge(
    mut laser_query: Query<&mut Laser>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
) {
    for mut laser in &mut laser_query {
        laser.timer.tick(time.delta());
        let mut normal_visibility = visibility_query.get_mut(laser.normal).unwrap();
        normal_visibility.is_visible = !laser.charging();
        let mut charging_visibility = visibility_query.get_mut(laser.charging).unwrap();
        charging_visibility.is_visible = laser.charging();
    }
}

fn attack(
    laser_query: Query<(&Position, &Laser), Without<Player>>,
    mut player_query: Query<&Position, With<Player>>,
    mut visibility_query: Query<&mut Visibility>,
    mut commands: Commands,
) {
    let player_position = player_query.single_mut();
    for (laser_position, laser) in &laser_query {
        let mut visibility = visibility_query.get_mut(laser.ray).unwrap();
        visibility.is_visible = laser.shooting();
        let aligned = match laser.axis {
            Axis::Vertical => laser_position.vec.x == player_position.vec.x,
        };
        if laser.shooting() && aligned {
            commands.insert_resource(NextState(AppState::Defeat));
        }
    }
}
