use bevy::{prelude::*, utils::HashSet};

use crate::palette;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_startup_system(setup)
            .add_system(to_world);
    }
}

fn setup(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in -1..=1 {
        for j in -1..=1 {
            commands.spawn(MaterialMeshBundle {
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

#[derive(Resource, Default)]
pub struct Board {
    pub tiles: HashSet<IVec2>,
}

#[derive(Component)]
pub struct Position {
    pub vec: IVec2,
}

impl Position {
    pub fn new(vec: IVec2) -> Self {
        Self { vec }
    }

    pub fn from_xy(x: i32, y: i32) -> Self {
        Self {
            vec: IVec2::new(x, y),
        }
    }
}

fn to_world(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.vec.x as f32;
        transform.translation.z = -position.vec.y as f32;
    }
}
