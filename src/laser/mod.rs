mod visuals;

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{board::Position, palette, player::Player, AppState};

use self::visuals::{ray_blueprint, turrets_blueprint, Visuals, VisualsPlugin};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VisualsPlugin)
            .add_enter_system(AppState::Setup, enter_setup)
            .add_enter_system(AppState::Teardown, enter_teardown)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Game)
                    .with_system(mode)
                    .with_system(movement)
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
    laser(
        &mut commands,
        IVec2::ZERO,
        Axis::Vertical,
        true,
        vec![
            Phase::new(Mode::Charging, 0.8),
            Phase::new(Mode::Shooting, 1.0),
            Phase::new(Mode::Ready, 1.8),
            Phase::new(Mode::Charging, 2.3),
            Phase::new(Mode::Shooting, 2.5),
            Phase::new(Mode::Ready, 3.3),
            Phase::new(Mode::Charging, 3.8),
            Phase::new(Mode::Shooting, 4.0),
            Phase::new(Mode::Ready, 5.0),
            Phase::new(Mode::Charging, 5.5),
            Phase::new(Mode::Shooting, 5.7),
            Phase::new(Mode::Ready, 6.5),
            Phase::new(Mode::Charging, 7.0),
            Phase::new(Mode::Shooting, 7.2),
            Phase::new(Mode::Ready, 8.0),
            Phase::new(Mode::Charging, 8.5),
            Phase::new(Mode::Shooting, 8.7),
        ],
        &mut meshes,
        &mut materials,
    );
    laser(
        &mut commands,
        IVec2::new(0, 0),
        Axis::Horizontal,
        false,
        vec![
            Phase::new(Mode::Ready, 6.5),
            Phase::new(Mode::Charging, 7.0),
            Phase::new(Mode::Shooting, 7.2),
        ],
        &mut meshes,
        &mut materials,
    );
    for i in [-1, 1] {
        laser(
            &mut commands,
            IVec2::new(0, i),
            Axis::Horizontal,
            false,
            vec![
                Phase::new(Mode::Ready, 5.0),
                Phase::new(Mode::Charging, 5.5),
                Phase::new(Mode::Shooting, 5.7),
                Phase::new(Mode::Ready, 8.0),
                Phase::new(Mode::Charging, 8.5),
                Phase::new(Mode::Shooting, 8.7),
            ],
            &mut meshes,
            &mut materials,
        );
    }
}

fn laser(
    commands: &mut Commands,
    position: IVec2,
    axis: Axis,
    mobile: bool,
    phases: Vec<Phase>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let normal = turrets_blueprint(commands, palette::DARK_YELLOW, meshes, materials);
    let charging = turrets_blueprint(commands, palette::DARK_RED, meshes, materials);
    let ray = ray_blueprint(commands, meshes, materials);
    let rotation = match axis {
        Axis::Horizontal => PI / 2.0,
        Axis::Vertical => 0.0,
    };
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_y(rotation)),
        ))
        .insert_bundle(VisibilityBundle::default())
        .insert(Position::new(position))
        .insert(Laser::new(axis, mobile, phases))
        .insert(Visuals::new(normal, charging, ray))
        .push_children(&[normal, charging, ray]);
}

fn enter_teardown(query: Query<Entity, With<Laser>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct Laser {
    pub axis: Axis,
    pub mobile: bool,
    pub phases: Vec<Phase>,
    pub elapsed: Duration,
}

impl Laser {
    pub fn new(axis: Axis, mobile: bool, phases: Vec<Phase>) -> Self {
        Self {
            axis,
            mobile,
            phases,
            elapsed: Duration::ZERO,
        }
    }

    pub fn mode(&self) -> Mode {
        match self.phases.first() {
            Some(phase) => phase.mode,
            None => Mode::Ready,
        }
    }
}

enum Axis {
    Horizontal,
    Vertical,
}

struct Phase {
    pub mode: Mode,
    pub duration: Duration,
}

impl Phase {
    pub fn new(mode: Mode, seconds: f32) -> Self {
        Self {
            mode,
            duration: Duration::from_secs_f32(seconds),
        }
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Ready,
    Charging,
    Shooting,
}

fn mode(mut query: Query<&mut Laser>, time: Res<Time>) {
    for mut laser in &mut query {
        laser.elapsed += time.delta();
        while let Some(phase) = laser.phases.first() {
            if phase.duration >= laser.elapsed {
                break;
            }
            laser.phases.remove(0);
        }
    }
}

fn movement(
    mut laser_query: Query<(&mut Position, &Laser), Without<Player>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_position = player_query.single().vec;
    for (mut laser_position, laser) in &mut laser_query {
        if laser.mobile && matches!(laser.mode(), Mode::Ready) {
            match laser.axis {
                Axis::Horizontal => laser_position.vec.y = player_position.y,
                Axis::Vertical => laser_position.vec.x = player_position.x,
            }
        }
    }
}

fn attack(
    laser_query: Query<(&Position, &Laser), Without<Player>>,
    mut player_query: Query<&Position, With<Player>>,
    mut commands: Commands,
) {
    let player_position = player_query.single_mut();
    for (laser_position, laser) in &laser_query {
        let aligned = match laser.axis {
            Axis::Horizontal => laser_position.vec.y == player_position.vec.y,
            Axis::Vertical => laser_position.vec.x == player_position.vec.x,
        };
        if matches!(laser.mode(), Mode::Shooting) && aligned {
            commands.insert_resource(NextState(AppState::Defeat));
        }
    }
}
