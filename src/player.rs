use bevy::prelude::*;

use crate::{
    board::{Board, Position},
    palette, AppState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(
                SystemSet::on_update(AppState::Start)
                    .with_system(start_trigger)
                    .with_system(movement),
            )
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement))
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(movement));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
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
        },
        Position::from_xy(0, 0),
        Player,
    ));
}

fn enter_setup(mut query: Query<&mut Position, With<Player>>) {
    let mut position = query.single_mut();
    position.vec = IVec2::ZERO;
}

#[derive(Component)]
pub struct Player;

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
