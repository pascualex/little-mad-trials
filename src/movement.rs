use bevy::{prelude::*, utils::HashSet};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_system(movement)
            .add_system(to_world);
    }
}

#[derive(Default)]
pub struct Board {
    pub tiles: HashSet<IVec2>,
}

#[derive(Component)]
pub struct Position {
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

fn to_world(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.vec.x as f32;
        transform.translation.z = -position.vec.y as f32;
    }
}
