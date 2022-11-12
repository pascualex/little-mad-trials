mod visuals;

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{background::Countdown, board::Position, palette, player::Player, AppState};

use self::visuals::{ray_blueprint, turrets_blueprint, Visuals, VisualsPlugin};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VisualsPlugin)
            .add_enter_system(AppState::Setup, enter_setup)
            .add_enter_system(AppState::Teardown, enter_teardown)
            .add_system(mode.run_in_state(AppState::Game).label("mode"))
            .add_system(
                movement
                    .run_in_state(AppState::Game)
                    .label("movement")
                    .after("mode"),
            )
            .add_system(attack.run_in_state(AppState::Game).after("movement"));
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
            // first round
            Phase::new(Mode::Charging, 0.8),
            Phase::new(Mode::Shooting, 1.0),
            Phase::new(Mode::Ready, 1.8),
            Phase::new(Mode::Charging, 2.3),
            Phase::new(Mode::Shooting, 2.5),
            Phase::new(Mode::Ready, 3.3),
            Phase::new(Mode::Charging, 3.8),
            Phase::new(Mode::Shooting, 4.0),
            // second round
            Phase::new(Mode::Ready, 5.0),
            Phase::new(Mode::Charging, 5.5),
            Phase::new(Mode::Shooting, 5.7),
            Phase::new(Mode::Ready, 6.5),
            Phase::new(Mode::Charging, 7.0),
            Phase::new(Mode::Shooting, 7.2),
            Phase::new(Mode::Ready, 8.0),
            Phase::new(Mode::Charging, 8.5),
            Phase::new(Mode::Shooting, 8.7),
            // third round
            Phase::new(Mode::Ready, 9.7),
            Phase::new(Mode::Charging, 10.2),
            Phase::new(Mode::Shooting, 10.4),
            Phase::new(Mode::Ready, 10.7),
            Phase::new(Mode::Charging, 11.2),
            Phase::new(Mode::Shooting, 11.4),
            Phase::new(Mode::Ready, 11.7),
            Phase::new(Mode::Charging, 12.2),
            Phase::new(Mode::Shooting, 12.4),
            // fourth round
            Phase::new(Mode::Ready, 13.4),
            Phase::new(Mode::Charging, 13.9),
            Phase::new(Mode::Shooting, 14.1),
            Phase::new(Mode::Ready, 14.4),
            Phase::new(Mode::Charging, 14.9),
            Phase::new(Mode::Shooting, 15.1),
            Phase::new(Mode::Ready, 15.4),
            Phase::new(Mode::Charging, 15.9),
            Phase::new(Mode::Shooting, 16.1),
            // fifth round
            Phase::new(Mode::Ready, 17.1),
            Phase::new(Mode::Charging, 17.6),
            Phase::new(Mode::Shooting, 17.8),
            Phase::new(Mode::Ready, 17.8),
            Phase::new(Mode::Charging, 18.3),
            Phase::new(Mode::Shooting, 18.5),
            Phase::new(Mode::Ready, 18.5),
            Phase::new(Mode::Charging, 19.0),
            Phase::new(Mode::Shooting, 19.2),
            Phase::new(Mode::Ready, 19.2),
            Phase::new(Mode::Charging, 19.7),
            Phase::new(Mode::Shooting, 19.9),
            Phase::new(Mode::Ready, 19.9),
            Phase::new(Mode::Charging, 20.4),
            Phase::new(Mode::Shooting, 20.6),
        ],
        &mut meshes,
        &mut materials,
    );
    laser(
        &mut commands,
        IVec2::new(0, -1),
        Axis::Horizontal,
        false,
        vec![
            // second round
            Phase::new(Mode::Ready, 5.0),
            Phase::new(Mode::Charging, 5.5),
            Phase::new(Mode::Shooting, 5.7),
            Phase::new(Mode::Ready, 8.0),
            Phase::new(Mode::Charging, 8.5),
            Phase::new(Mode::Shooting, 8.7),
            // fourth round
            Phase::new(Mode::Ready, 14.4),
            Phase::new(Mode::Charging, 14.9),
            Phase::new(Mode::Shooting, 15.1),
            Phase::new(Mode::Ready, 15.4),
            Phase::new(Mode::Charging, 15.9),
            Phase::new(Mode::Shooting, 20.8),
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
            // second round
            Phase::new(Mode::Ready, 6.5),
            Phase::new(Mode::Charging, 7.0),
            Phase::new(Mode::Shooting, 7.2),
            // fourth round
            Phase::new(Mode::Ready, 13.4),
            Phase::new(Mode::Charging, 13.9),
            Phase::new(Mode::Shooting, 14.1),
            Phase::new(Mode::Ready, 14.4),
            Phase::new(Mode::Charging, 14.9),
            Phase::new(Mode::Shooting, 15.1),
        ],
        &mut meshes,
        &mut materials,
    );
    laser(
        &mut commands,
        IVec2::new(0, 1),
        Axis::Horizontal,
        false,
        vec![
            // second round
            Phase::new(Mode::Ready, 5.0),
            Phase::new(Mode::Charging, 5.5),
            Phase::new(Mode::Shooting, 5.7),
            Phase::new(Mode::Ready, 8.0),
            Phase::new(Mode::Charging, 8.5),
            Phase::new(Mode::Shooting, 8.7),
            // fourth round
            Phase::new(Mode::Ready, 13.4),
            Phase::new(Mode::Charging, 13.9),
            Phase::new(Mode::Shooting, 14.1),
            Phase::new(Mode::Ready, 15.4),
            Phase::new(Mode::Charging, 15.9),
            Phase::new(Mode::Shooting, 20.8),
        ],
        &mut meshes,
        &mut materials,
    );
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
}

impl Laser {
    pub fn new(axis: Axis, mobile: bool, phases: Vec<Phase>) -> Self {
        Self {
            axis,
            mobile,
            phases,
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
    pub end: Duration,
}

impl Phase {
    pub fn new(mode: Mode, seconds: f32) -> Self {
        Self {
            mode,
            end: Duration::from_secs_f32(seconds),
        }
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Ready,
    Charging,
    Shooting,
}

fn mode(mut query: Query<&mut Laser>, countdown: Res<Countdown>) {
    for mut laser in &mut query {
        let Some(phase) = laser.phases.first() else {
            continue;
        };
        if countdown.timer.elapsed() >= phase.end {
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
