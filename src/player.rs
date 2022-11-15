use bevy::prelude::*;

use crate::{
    board::{Board, BoardMode, Position, HIDDEN_HEIGHT},
    laser, palette,
    phases::{Phase, Phases},
    AppState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Health>()
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system_set(
                SystemSet::on_update(AppState::Start)
                    .with_system(start_trigger)
                    .with_system(movement),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(movement)
                    .with_system(defeat.after(laser::attack)),
            )
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(movement))
            .add_system(health_visuals);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let alive = commands
        .spawn(MaterialMeshBundle {
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
        .id();
    let dead = commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.8))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_BLACK,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.4, 0.0),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .id();
    let root = (
        TransformBundle::from_transform(Transform::from_xyz(0.0, HIDDEN_HEIGHT, 0.0)),
        VisibilityBundle::default(),
        Position::from_xy(0, 0),
        Player::new(alive, dead),
        Phases::new(BoardMode::Hidden),
    );
    commands.spawn(root).push_children(&[alive, dead]);
}

fn enter_setup(
    mut query: Query<(&mut Position, &mut Phases<BoardMode>), With<Player>>,
    mut health: ResMut<Health>,
) {
    let (mut position, mut phases) = query.single_mut();
    position.vec = IVec2::ZERO;
    phases.reset(vec![
        Phase::new(BoardMode::Entering, 1.0), // 1.0
        Phase::new(BoardMode::Shown, 0.0),    // final
    ]);
    health.dead = false;
}

fn enter_teardown(mut query: Query<&mut Phases<BoardMode>, With<Player>>) {
    let mut phases = query.single_mut();
    phases.reset(vec![
        Phase::new(BoardMode::Exiting, 1.0), // 1.0
        Phase::new(BoardMode::Hidden, 0.0),  // final
    ]);
}

#[derive(Resource, Default)]
pub struct Health {
    pub dead: bool,
}

#[derive(Component)]
pub struct Player {
    pub alive: Entity,
    pub dead: Entity,
}

impl Player {
    pub fn new(alive: Entity, dead: Entity) -> Self {
        Self { alive, dead }
    }
}

pub fn movement(
    mut query: Query<&mut Position, With<Player>>,
    board: Res<Board>,
    input: Res<Input<KeyCode>>,
) {
    let direction = IVec2::new(
        input.just_pressed(KeyCode::Right) as i32 - input.just_pressed(KeyCode::Left) as i32,
        input.just_pressed(KeyCode::Up) as i32 - input.just_pressed(KeyCode::Down) as i32,
    );
    let mut position = query.single_mut();
    let new_position = position.vec + direction;
    if board.tiles.contains(&new_position) {
        position.vec = new_position;
    }
}

fn start_trigger(query: Query<&Position, With<Player>>, mut state: ResMut<State<AppState>>) {
    let position = query.single();
    if position.vec != IVec2::ZERO {
        state.overwrite_set(AppState::Game).unwrap();
    }
}

fn defeat(health: Res<Health>, mut state: ResMut<State<AppState>>) {
    if health.dead {
        state.overwrite_set(AppState::Defeat).unwrap();
    }
}

fn health_visuals(
    player_query: Query<&Player>,
    mut visibility_query: Query<&mut Visibility>,
    health: Res<Health>,
) {
    let player = player_query.single();
    let mut alive_visibility = visibility_query.get_mut(player.alive).unwrap();
    alive_visibility.is_visible = !health.dead;
    let mut dead_visibility = visibility_query.get_mut(player.dead).unwrap();
    dead_visibility.is_visible = health.dead;
}
