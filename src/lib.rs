mod palette;

use bevy::{prelude::*, utils::HashSet};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_startup_system(setup)
            .add_system(movement)
            .add_system(board_to_world);
    }
}

fn setup(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    let size = 11.0;
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ZERO)
            .looking_at(Vec3::new(-1.0, -3.0, -2.0), Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 32_000.0,
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -size,
                right: size,
                bottom: -size,
                top: size,
                near: -size,
                far: size,
                ..default()
            },
            ..default()
        },
        ..default()
    });
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
        .insert(Position::from_xy(0, 0));
    for i in -1..=1 {
        for j in -1..=1 {
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.92, 4.0, 0.92))),
                material: materials.add(StandardMaterial {
                    base_color: palette::LIGHT_WHITE,
                    metallic: 0.1,
                    perceptual_roughness: 0.7,
                    reflectance: 0.3,
                    ..default()
                }),
                transform: Transform::from_xyz(j as f32, -2.0, -i as f32),
                ..default()
            });
            board.tiles.insert(IVec2::new(j, i));
        }
    }
}

#[derive(Default)]
struct Board {
    pub tiles: HashSet<IVec2>,
}

#[derive(Component)]
struct Position {
    pub vec: IVec2,
}

impl Position {
    pub fn from_xy(x: i32, y: i32) -> Self {
        Self {
            vec: IVec2::new(x, y),
        }
    }
}

fn movement(mut query: Query<&mut Position>, board: Res<Board>, input: Res<Input<KeyCode>>) {
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

fn board_to_world(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.vec.x as f32;
        transform.translation.z = -position.vec.y as f32;
    }
}
