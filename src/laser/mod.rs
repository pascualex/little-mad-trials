mod content;
mod visuals;

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    background,
    board::{BoardMode, Position, HIDDEN_HEIGHT},
    palette,
    phases::{self, Phase, Phases},
    player::{self, Health, Player},
    AppState,
};

use self::{
    content::{
        lower_laser_board_phases, lower_laser_phases, middle_laser_board_phases,
        middle_laser_phases, mobile_laser_board_phases, mobile_laser_phases,
        upper_laser_board_phases, upper_laser_phases,
    },
    visuals::{ray_blueprint, turrets_blueprint, Visuals, VisualsPlugin},
};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VisualsPlugin)
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(movement.after(phases::transition::<LaserMode>))
                    .with_system(attack.after(movement).after(player::movement)),
            )
            .add_system_set(SystemSet::on_enter(AppState::Victory).with_system(enter_teardown))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system(phases::transition::<LaserMode>.after(background::countdown));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    laser(
        IVec2::ZERO,
        Axis::Vertical,
        true,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    laser(
        IVec2::new(0, 1),
        Axis::Horizontal,
        false,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    laser(
        IVec2::new(0, 0),
        Axis::Horizontal,
        false,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    laser(
        IVec2::new(0, -1),
        Axis::Horizontal,
        false,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn laser(
    position: IVec2,
    axis: Axis,
    mobile: bool,
    commands: &mut Commands,
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
    let model = (
        TransformBundle::from_transform(
            Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_y(rotation)),
        ),
        VisibilityBundle::default(),
    );
    let root = (
        TransformBundle::from_transform(Transform::from_xyz(0.0, HIDDEN_HEIGHT, 0.0)),
        VisibilityBundle::default(),
        Position::new(position),
        Laser::new(axis, mobile),
        Visuals::new(normal, charging, ray),
        Phases::new(BoardMode::Hidden),
        Phases::new(LaserMode::Ready),
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(model).push_children(&[normal, charging, ray]);
    });
}

fn enter_setup(
    mut query: Query<(
        &mut Position,
        &Laser,
        &mut Phases<BoardMode>,
        &mut Phases<LaserMode>,
    )>,
) {
    for (mut position, laser, mut board_phases, mut laser_phases) in &mut query {
        if laser.mobile {
            position.vec = IVec2::ZERO;
        };
        let vec = match laser.mobile {
            true => vec![
                Phase::new(BoardMode::Entering, 1.0), // 1.0
                Phase::new(BoardMode::Shown, 0.0),    // final
            ],
            false => vec![
                Phase::new(BoardMode::Hidden, 0.0), // final
            ],
        };
        board_phases.reset(vec);
        laser_phases.reset(vec![
            Phase::new(LaserMode::Ready, 0.0), // final
        ]);
    }
}

fn enter_start(
    mut query: Query<(
        &Position,
        &Laser,
        &mut Phases<BoardMode>,
        &mut Phases<LaserMode>,
    )>,
) {
    for (position, laser, mut board_phases, mut laser_phases) in &mut query {
        let vec = match laser.mobile {
            true => mobile_laser_board_phases(),
            false => match position.vec.y {
                1 => upper_laser_board_phases(),
                0 => middle_laser_board_phases(),
                -1 => lower_laser_board_phases(),
                _ => unreachable!(),
            },
        };
        board_phases.reset(vec);
        let vec = match laser.mobile {
            true => mobile_laser_phases(),
            false => match position.vec.y {
                1 => upper_laser_phases(),
                0 => middle_laser_phases(),
                -1 => lower_laser_phases(),
                _ => unreachable!(),
            },
        };
        laser_phases.reset(vec);
    }
}

fn enter_teardown(mut query: Query<(&mut Phases<BoardMode>, &mut Phases<LaserMode>), With<Laser>>) {
    for (mut board_phases, mut laser_phases) in &mut query {
        board_phases.reset(vec![
            Phase::new(BoardMode::Exiting, 1.0), // 1.0
            Phase::new(BoardMode::Hidden, 0.0),  // final
        ]);
        laser_phases.reset(vec![
            Phase::new(LaserMode::Ready, 0.0), // final
        ]);
    }
}

#[derive(Component)]
pub struct Laser {
    pub axis: Axis,
    pub mobile: bool,
}

impl Laser {
    pub fn new(axis: Axis, mobile: bool) -> Self {
        Self { axis, mobile }
    }
}

pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub enum LaserMode {
    Ready,
    Charging,
    Shooting,
}

fn movement(
    mut laser_query: Query<(&mut Position, &Laser, &Phases<LaserMode>), Without<Player>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_position = player_query.single().vec;
    for (mut laser_position, laser, phases) in &mut laser_query {
        if laser.mobile && matches!(phases.mode(), LaserMode::Ready) {
            match laser.axis {
                Axis::Horizontal => laser_position.vec.y = player_position.y,
                Axis::Vertical => laser_position.vec.x = player_position.x,
            }
        }
    }
}

pub fn attack(
    laser_query: Query<(&Position, &Laser, &Phases<LaserMode>), Without<Player>>,
    mut player_query: Query<&Position, With<Player>>,
    mut health: ResMut<Health>,
) {
    let player_position = player_query.single_mut();
    for (laser_position, laser, phases) in &laser_query {
        let aligned = match laser.axis {
            Axis::Horizontal => laser_position.vec.y == player_position.vec.y,
            Axis::Vertical => laser_position.vec.x == player_position.vec.x,
        };
        if matches!(phases.mode(), LaserMode::Shooting) && aligned {
            health.dead = true;
        }
    }
}
