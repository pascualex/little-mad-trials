mod phases;
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

use self::{
    phases::{lower_laser_phases, middle_laser_phases, moving_laser_phases, upper_laser_phases},
    visuals::{ray_blueprint, turrets_blueprint, Visuals, VisualsPlugin},
};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VisualsPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(movement.after(phase))
                    .with_system(attack.after(movement).after(player::movement)),
            )
            .add_system(phase);
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
        moving_laser_phases(),
        &mut meshes,
        &mut materials,
    );
    laser(
        &mut commands,
        IVec2::new(0, 1),
        Axis::Horizontal,
        false,
        upper_laser_phases(),
        &mut meshes,
        &mut materials,
    );
    laser(
        &mut commands,
        IVec2::new(0, 0),
        Axis::Horizontal,
        false,
        middle_laser_phases(),
        &mut meshes,
        &mut materials,
    );
    laser(
        &mut commands,
        IVec2::new(0, -1),
        Axis::Horizontal,
        false,
        lower_laser_phases(),
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
    pub phase_start: Duration,
}

impl Laser {
    pub fn new(axis: Axis, mobile: bool, phases: Vec<Phase>) -> Self {
        Self {
            axis,
            mobile,
            phases,
            phase_start: Duration::ZERO,
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

pub struct Phase {
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
pub enum Mode {
    Ready,
    Charging,
    Shooting,
}

fn phase(mut query: Query<&mut Laser>, countdown: Res<Countdown>) {
    for mut laser in &mut query {
        let duration = match laser.phases.first() {
            Some(phase) => phase.duration,
            None => continue,
        };
        if countdown.timer.elapsed() >= laser.phase_start + duration {
            laser.phase_start += duration;
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
