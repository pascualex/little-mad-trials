use bevy::prelude::*;

use crate::{
    board::{Board, Position},
    palette,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_startup_system(setup)
            .add_system(movement);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
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
        .insert(Position::from_xy(0, 0))
        .insert(Player);
}

#[derive(Component)]
pub struct Player;

fn movement(
    mut query: Query<&mut Position, With<Player>>,
    board: Res<Board>,
    input: Res<Input<KeyCode>>,
) {
    let mut direction = IVec2::new(
        input.just_pressed(KeyCode::Right) as i32 - input.just_pressed(KeyCode::Left) as i32,
        input.just_pressed(KeyCode::Up) as i32 - input.just_pressed(KeyCode::Down) as i32,
    );
    if direction.x != 0 {
        direction.y = 0;
    }
    let mut position = query.single_mut();
    let new_position = position.vec + direction;
    if board.tiles.contains(&new_position) {
        position.vec = new_position;
    }
}
