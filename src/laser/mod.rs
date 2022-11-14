mod visuals;

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{
    background::Countdown,
    board::Position,
    palette,
    player::{self, Player},
    AppState,
};

use self::visuals::{ray_blueprint, turrets_blueprint, Visuals, VisualsPlugin};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VisualsPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(movement.after(mode))
                    .with_system(attack.after(movement).after(player::movement)),
            )
            .add_system(mode);
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
            Phase::new(Mode::Charging, 0.2),
            Phase::new(Mode::Shooting, 0.4),
            Phase::new(Mode::Ready, 1.2),
            Phase::new(Mode::Charging, 1.7),
            Phase::new(Mode::Shooting, 1.9),
            Phase::new(Mode::Ready, 2.7),
            Phase::new(Mode::Charging, 3.2),
            Phase::new(Mode::Shooting, 3.4),
            // second round
            Phase::new(Mode::Ready, 4.4),
            Phase::new(Mode::Charging, 4.9),
            Phase::new(Mode::Shooting, 5.1),
            Phase::new(Mode::Ready, 5.9),
            Phase::new(Mode::Charging, 6.4),
            Phase::new(Mode::Shooting, 6.6),
            Phase::new(Mode::Ready, 7.4),
            Phase::new(Mode::Charging, 7.9),
            Phase::new(Mode::Shooting, 8.1),
            // third round
            Phase::new(Mode::Ready, 9.1),
            Phase::new(Mode::Charging, 9.6),
            Phase::new(Mode::Shooting, 9.8),
            Phase::new(Mode::Ready, 10.1),
            Phase::new(Mode::Charging, 10.6),
            Phase::new(Mode::Shooting, 10.8),
            Phase::new(Mode::Ready, 11.1),
            Phase::new(Mode::Charging, 11.6),
            Phase::new(Mode::Shooting, 11.8),
            // fourth round
            Phase::new(Mode::Ready, 12.8),
            Phase::new(Mode::Charging, 13.3),
            Phase::new(Mode::Shooting, 13.5),
            Phase::new(Mode::Ready, 13.8),
            Phase::new(Mode::Charging, 14.3),
            Phase::new(Mode::Shooting, 14.5),
            Phase::new(Mode::Ready, 14.8),
            Phase::new(Mode::Charging, 15.3),
            Phase::new(Mode::Shooting, 15.5),
            // fifth round
            Phase::new(Mode::Ready, 16.5),
            Phase::new(Mode::Charging, 17.0),
            Phase::new(Mode::Shooting, 17.2),
            Phase::new(Mode::Ready, 17.2),
            Phase::new(Mode::Charging, 17.7),
            Phase::new(Mode::Shooting, 17.9),
            Phase::new(Mode::Ready, 17.9),
            Phase::new(Mode::Charging, 18.4),
            Phase::new(Mode::Shooting, 18.6),
            Phase::new(Mode::Ready, 18.6),
            Phase::new(Mode::Charging, 19.1),
            Phase::new(Mode::Shooting, 19.3),
            Phase::new(Mode::Ready, 19.3),
            Phase::new(Mode::Charging, 19.8),
            Phase::new(Mode::Shooting, 20.0),
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
            Phase::new(Mode::Ready, 4.4),
            Phase::new(Mode::Charging, 4.9),
            Phase::new(Mode::Shooting, 5.1),
            Phase::new(Mode::Ready, 7.4),
            Phase::new(Mode::Charging, 7.9),
            Phase::new(Mode::Shooting, 8.1),
            // fourth round
            Phase::new(Mode::Ready, 13.8),
            Phase::new(Mode::Charging, 14.3),
            Phase::new(Mode::Shooting, 14.5),
            Phase::new(Mode::Ready, 14.8),
            Phase::new(Mode::Charging, 15.3),
            Phase::new(Mode::Shooting, 20.0),
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
            Phase::new(Mode::Ready, 5.9),
            Phase::new(Mode::Charging, 6.4),
            Phase::new(Mode::Shooting, 6.6),
            // fourth round
            Phase::new(Mode::Ready, 12.8),
            Phase::new(Mode::Charging, 13.3),
            Phase::new(Mode::Shooting, 13.5),
            Phase::new(Mode::Ready, 13.8),
            Phase::new(Mode::Charging, 14.3),
            Phase::new(Mode::Shooting, 14.5),
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
            Phase::new(Mode::Ready, 4.4),
            Phase::new(Mode::Charging, 4.9),
            Phase::new(Mode::Shooting, 5.1),
            Phase::new(Mode::Ready, 7.4),
            Phase::new(Mode::Charging, 7.9),
            Phase::new(Mode::Shooting, 8.1),
            // fourth round
            Phase::new(Mode::Ready, 12.8),
            Phase::new(Mode::Charging, 13.3),
            Phase::new(Mode::Shooting, 13.5),
            Phase::new(Mode::Ready, 14.8),
            Phase::new(Mode::Charging, 15.3),
            Phase::new(Mode::Shooting, 20.0),
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
    let rotation = match axis {
        Axis::Horizontal => PI / 2.0,
        Axis::Vertical => 0.0,
    };
    let normal = turrets_blueprint(commands, palette::DARK_YELLOW, meshes, materials);
    let charging = turrets_blueprint(commands, palette::DARK_RED, meshes, materials);
    let ray = ray_blueprint(commands, meshes, materials);
    let root = (
        TransformBundle::from_transform(
            Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_y(rotation)),
        ),
        VisibilityBundle::default(),
        Position::new(position),
        Laser::new(axis, mobile, phases),
        Visuals::new(normal, charging, ray),
    );
    commands.spawn(root).push_children(&[normal, charging, ray]);
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
    mut state: ResMut<State<AppState>>,
) {
    let player_position = player_query.single_mut();
    for (laser_position, laser) in &laser_query {
        let aligned = match laser.axis {
            Axis::Horizontal => laser_position.vec.y == player_position.vec.y,
            Axis::Vertical => laser_position.vec.x == player_position.vec.x,
        };
        if matches!(laser.mode(), Mode::Shooting) && aligned {
            state.overwrite_set(AppState::Defeat).unwrap();
        }
    }
}
