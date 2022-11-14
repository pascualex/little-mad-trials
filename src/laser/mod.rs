mod content;
mod visuals;

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    board::{BoardMode, Position},
    palette,
    phases::{self, Phase, Phases},
    player::{self, Player},
    AppState,
};

use self::{
    content::{lower_laser_phases, middle_laser_phases, moving_laser_phases, upper_laser_phases},
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
                    .with_system(movement.after(phases::transition::<LaserMode>))
                    .with_system(attack.after(movement).after(player::movement)),
            )
            .add_system(phases::transition::<LaserMode>);
    }
}

fn enter_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    laser(
        IVec2::ZERO,
        Laser::new(Axis::Vertical, true),
        vec![
            Phase::new(BoardMode::Shown, 20.0),  // 20.0
            Phase::new(BoardMode::Exiting, 1.0), // 21.0
        ],
        moving_laser_phases(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    laser(
        IVec2::new(0, 1),
        Laser::new(Axis::Horizontal, false),
        vec![
            Phase::new(BoardMode::Hidden, 3.4),   // 3.4
            Phase::new(BoardMode::Entering, 1.0), // 4.4
            Phase::new(BoardMode::Shown, 3.7),    // 8.1
            Phase::new(BoardMode::Exiting, 1.0),  // 9.1
            Phase::new(BoardMode::Hidden, 2.7),   // 11.8
            Phase::new(BoardMode::Entering, 1.0), // 12.8
            Phase::new(BoardMode::Shown, 7.2),    // 20.0
            Phase::new(BoardMode::Exiting, 1.0),  // 21.0
        ],
        upper_laser_phases(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    laser(
        IVec2::new(0, 0),
        Laser::new(Axis::Horizontal, false),
        vec![
            Phase::new(BoardMode::Hidden, 4.9),   // 4.9
            Phase::new(BoardMode::Entering, 1.0), // 5.9
            Phase::new(BoardMode::Shown, 0.7),    // 6.6
            Phase::new(BoardMode::Exiting, 1.0),  // 7.6
            Phase::new(BoardMode::Hidden, 4.2),   // 11.8
            Phase::new(BoardMode::Entering, 1.0), // 12.8
            Phase::new(BoardMode::Shown, 2.7),    // 14.5
            Phase::new(BoardMode::Exiting, 1.0),  // 15.5
        ],
        middle_laser_phases(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    laser(
        IVec2::new(0, -1),
        Laser::new(Axis::Horizontal, false),
        vec![
            Phase::new(BoardMode::Hidden, 3.4),   // 3.4
            Phase::new(BoardMode::Entering, 1.0), // 4.4
            Phase::new(BoardMode::Shown, 3.7),    // 8.1
            Phase::new(BoardMode::Exiting, 1.0),  // 9.1
            Phase::new(BoardMode::Hidden, 3.7),   // 12.8
            Phase::new(BoardMode::Entering, 1.0), // 13.8
            Phase::new(BoardMode::Shown, 6.2),    // 20.0
            Phase::new(BoardMode::Exiting, 1.0),  // 21.0
        ],
        lower_laser_phases(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

fn laser(
    position: IVec2,
    laser: Laser,
    board_phases: Vec<Phase<BoardMode>>,
    laser_phases: Vec<Phase<LaserMode>>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let rotation = match laser.axis {
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
        TransformBundle::default(),
        VisibilityBundle::default(),
        Position::new(position),
        laser,
        Visuals::new(normal, charging, ray),
        Phases::new(board_phases),
        Phases::new(laser_phases),
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(model).push_children(&[normal, charging, ray]);
    });
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
}

impl Laser {
    pub fn new(axis: Axis, mobile: bool) -> Self {
        Self { axis, mobile }
    }
}

enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Default, Clone, Copy)]
pub enum LaserMode {
    #[default]
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

fn attack(
    laser_query: Query<(&Position, &Laser, &Phases<LaserMode>), Without<Player>>,
    mut player_query: Query<&Position, With<Player>>,
    mut state: ResMut<State<AppState>>,
) {
    let player_position = player_query.single_mut();
    for (laser_position, laser, phases) in &laser_query {
        let aligned = match laser.axis {
            Axis::Horizontal => laser_position.vec.y == player_position.vec.y,
            Axis::Vertical => laser_position.vec.x == player_position.vec.x,
        };
        if matches!(phases.mode(), LaserMode::Shooting) && aligned {
            state.overwrite_set(AppState::Defeat).unwrap();
        }
    }
}
