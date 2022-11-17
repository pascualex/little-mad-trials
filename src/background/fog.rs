use std::f32::consts::PI;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{material_from_color, palette};

pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(40.0, 40.0)))),
            material: materials.add(material_from_color(palette::DARK_PINK)),
            transform: Transform::from_xyz(0.0, -4.0, 10.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}
