use std::f32::consts::PI;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::palette;

pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<FogMaterial>::default())
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FogMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(40.0, 40.0)))),
            material: materials.add(FogMaterial {
                color: palette::DARK_PINK,
                alpha_mode: AlphaMode::Blend,
            }),
            transform: Transform::from_xyz(0.0, -4.0, 10.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

#[derive(Clone, AsBindGroup, TypeUuid, Debug)]
#[uuid = "fec7aad3-dd4b-43d6-be0b-a56cf4349038"]
struct FogMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}

impl Material for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
