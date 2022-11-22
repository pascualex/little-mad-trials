use bevy::prelude::*;

use crate::{
    board::{Board, BoardMode, Position, HIDDEN_HEIGHT},
    laser, material_from_color, palette,
    phases::{Phase, Phases},
    AppState,
};

const PLAYER_ALIVE_COLORS: [Color; 5] = [
    palette::DARK_BLUE,
    palette::DARK_GREEN,
    palette::DARK_PURPLE,
    palette::DARK_CYAN,
    palette::DARK_PINK,
];
const PLAYER_DEAD_COLOR: Color = palette::DARK_BLACK;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Health>()
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_enter(AppState::Defeat).with_system(enter_defeat))
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
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(movement));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model = commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.8))),
            material: materials.add(material_from_color(PLAYER_ALIVE_COLORS[0])),
            transform: Transform::from_xyz(0.0, 0.4, 0.0),
            ..default()
        })
        .id();
    let root = (
        SpatialBundle::from_transform(Transform::from_xyz(0.0, HIDDEN_HEIGHT, 0.0)),
        Position::from_xy(0, 0),
        Player::new(model),
        Phases::new(BoardMode::Hidden),
    );
    commands.spawn(root).push_children(&[model]);
}

fn enter_setup(
    mut player_query: Query<(&mut Position, &mut Player, &mut Phases<BoardMode>)>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    mut health: ResMut<Health>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (mut position, mut player, mut phases) = player_query.single_mut();
    position.vec = IVec2::ZERO;
    phases.reset(vec![
        Phase::new(BoardMode::Waiting, 0.4),  // 0.4
        Phase::new(BoardMode::Entering, 1.0), // 1.4
        Phase::new(BoardMode::Shown, 0.0),    // final
    ]);
    let mut handle = material_query.get_mut(player.model).unwrap();
    *handle = materials.add(material_from_color(PLAYER_ALIVE_COLORS[player.color]));
    let mut color = player.color;
    while color == player.color {
        color = fastrand::usize(..PLAYER_ALIVE_COLORS.len());
    }
    player.color = color;
    health.dead = false;
}

fn enter_defeat(
    player_query: Query<&Player>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = player_query.single();
    let mut handle = material_query.get_mut(player.model).unwrap();
    *handle = materials.add(material_from_color(PLAYER_DEAD_COLOR * 0.5));
}

fn enter_teardown(mut query: Query<(&Position, &mut Phases<BoardMode>), With<Player>>) {
    let (position, mut phases) = query.single_mut();
    let offset = position.vec.x - position.vec.y;
    phases.reset(vec![
        Phase::new(BoardMode::Waiting, 0.1 + offset as f32 * 0.05),
        Phase::new(BoardMode::Exiting, 1.0),
        Phase::new(BoardMode::Hidden, 0.0), // final
    ]);
}

#[derive(Resource, Default)]
pub struct Health {
    pub dead: bool,
}

#[derive(Component)]
pub struct Player {
    model: Entity,
    color: usize,
}

impl Player {
    pub fn new(alive: Entity) -> Self {
        Self {
            model: alive,
            color: 0,
        }
    }
}

pub fn movement(
    mut query: Query<&mut Position, With<Player>>,
    board: Res<Board>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let direction = IVec2::new(
        input.just_pressed(KeyCode::Right) as i32 - input.just_pressed(KeyCode::Left) as i32,
        input.just_pressed(KeyCode::Up) as i32 - input.just_pressed(KeyCode::Down) as i32,
    );
    if direction == IVec2::ZERO {
        return;
    }
    let mut position = query.single_mut();
    let new_position = position.vec + direction;
    if board.tiles.contains(&new_position) {
        position.vec = new_position;
        let sound = asset_server.load("sounds/step.ogg");
        audio.play_with_settings(sound, PlaybackSettings::ONCE.with_volume(0.05));
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
