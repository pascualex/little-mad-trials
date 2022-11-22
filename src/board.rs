use bevy::{prelude::*, utils::HashSet};

use crate::{
    background, material_from_color, palette,
    phases::{self, Phase, Phases},
    AppState,
};

pub const HIDDEN_HEIGHT: f32 = -5.0;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system(phases::transition::<BoardMode>.after(background::countdown))
            .add_system(to_world_xz)
            .add_system(to_world_y.after(phases::transition::<BoardMode>));
    }
}

fn setup(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.92, 40.0, 0.92))),
        material: materials.add(material_from_color(palette::DARK_BLACK)),
        transform: Transform::from_xyz(0.0, -20.0, 0.0),
        ..default()
    };
    for i in -1..=1 {
        for j in -1..=1 {
            let root = (
                SpatialBundle::from_transform(Transform::from_xyz(
                    j as f32,
                    HIDDEN_HEIGHT,
                    -i as f32,
                )),
                Tile,
                Phases::new(BoardMode::Hidden),
            );
            commands.spawn(root).with_children(|builder| {
                builder.spawn(model.clone());
            });
            board.tiles.insert(IVec2::new(j, i));
        }
    }
}

fn enter_setup(mut query: Query<(&Transform, &mut Phases<BoardMode>), With<Tile>>) {
    for (transform, mut phases) in &mut query {
        let offset = transform.translation.x + transform.translation.z;
        phases.reset(vec![
            Phase::new(BoardMode::Waiting, 0.4 + offset as f32 * 0.05),
            Phase::new(BoardMode::Entering, 1.0),
            Phase::new(BoardMode::Shown, 0.0), // final
        ]);
    }
}

fn enter_teardown(mut query: Query<(&Transform, &mut Phases<BoardMode>), With<Tile>>) {
    for (transform, mut phases) in &mut query {
        let offset = transform.translation.x + transform.translation.z;
        phases.reset(vec![
            Phase::new(BoardMode::Waiting, 0.1 + offset as f32 * 0.05),
            Phase::new(BoardMode::Exiting, 1.0),
            Phase::new(BoardMode::Hidden, 0.0), // final
        ]);
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

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Copy)]
pub enum BoardMode {
    Hidden,
    Entering,
    Shown,
    Exiting,
    Waiting,
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

fn to_world_xz(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.vec.x as f32;
        transform.translation.z = -position.vec.y as f32;
    }
}

fn to_world_y(mut query: Query<(&mut Transform, &Phases<BoardMode>)>) {
    for (mut transform, phases) in &mut query {
        let old_y = transform.translation.y;
        transform.translation.y = match phases.mode() {
            BoardMode::Hidden => HIDDEN_HEIGHT,
            BoardMode::Entering => HIDDEN_HEIGHT * (1.0 - ease(phases.progress)),
            BoardMode::Shown => 0.0,
            BoardMode::Exiting => {
                let new_y = HIDDEN_HEIGHT * ease(phases.progress);
                let both_above = old_y >= 0.0 && new_y >= 0.0;
                let both_bellow = old_y <= HIDDEN_HEIGHT && new_y <= HIDDEN_HEIGHT;
                match both_above || both_bellow {
                    true => new_y,
                    false => f32::min(old_y, new_y),
                }
            }
            BoardMode::Waiting => old_y,
        };
    }
}

const C: f32 = 1.7;

fn ease(x: f32) -> f32 {
    match x < 0.5 {
        true => ((2.0 * x).powi(2) * ((C + 1.0) * 2.0 * x - C)) / 2.0,
        false => ((2.0 - 2.0 * x).powi(2) * ((C + 1.0) * (x * 2.0 - 2.0) + C) + 2.0) / 2.0,
    }
}
