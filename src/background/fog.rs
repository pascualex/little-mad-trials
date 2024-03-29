use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices,
        render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef},
    },
};

use crate::board::HIDDEN_HEIGHT;

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
            mesh: meshes.add(Mesh::from(Fog::new(50.0, 100))),
            material: materials.add(FogMaterial::new()),
            transform: Transform::from_xyz(0.0, HIDDEN_HEIGHT + 1.0, 0.0),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

#[derive(Clone, AsBindGroup, TypeUuid, Debug)]
#[uuid = "fec7aad3-dd4b-43d6-be0b-a56cf4349038"]
struct FogMaterial {}

impl FogMaterial {
    fn new() -> Self {
        Self {}
    }
}

impl Material for FogMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }
}

#[derive(Debug, Copy, Clone)]
struct Fog {
    size: f32,
    num_vertices: u32,
}

impl Fog {
    pub fn new(size: f32, num_vertices: u32) -> Self {
        Self { size, num_vertices }
    }
}

impl From<Fog> for Mesh {
    fn from(fog: Fog) -> Self {
        let side = fog.size / 2.0;

        let mut uvs = Vec::new();
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        for i in 0..fog.num_vertices {
            for j in 0..fog.num_vertices {
                let uv = [
                    j as f32 / (fog.num_vertices - 1) as f32,
                    i as f32 / (fog.num_vertices - 1) as f32,
                ];
                uvs.push(uv);
                positions.push([uv[0] * fog.size - side, 0.0, uv[1] * fog.size - side]);
                normals.push([0.0, 1.0, 0.0]);
            }
        }

        let mut indices = Vec::new();
        for i in 0..(fog.num_vertices - 1) {
            for j in 0..(fog.num_vertices - 1) {
                let idx = i * fog.num_vertices + j;
                // top left triangle
                indices.push(idx);
                indices.push(idx + fog.num_vertices);
                indices.push(idx + fog.num_vertices + 1);
                // bottom right triangle
                indices.push(idx);
                indices.push(idx + fog.num_vertices + 1);
                indices.push(idx + 1);
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
