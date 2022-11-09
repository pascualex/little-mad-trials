use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{laser::Laser, palette, player::Player, AppState};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(charge)
                .with_system(attack)
                .into(),
        );
    }
}

pub fn turrets_blueprint(
    commands: &mut Commands,
    color: Color,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let top = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
        material: materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    };
    let bottom = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
        material: materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    };
    commands
        .spawn_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .with_children(|builder| {
            builder.spawn_bundle(top);
            builder.spawn_bundle(bottom);
        })
        .id()
}

pub fn ray_blueprint(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 4.0))),
            material: materials.add(StandardMaterial {
                base_color: palette::DARK_RED,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .id()
}

#[derive(Component)]
pub struct Visuals {
    pub normal: Entity,
    pub charging: Entity,
    pub ray: Entity,
}

impl Visuals {
    pub fn new(normal: Entity, charging: Entity, ray: Entity) -> Self {
        Self {
            normal,
            charging,
            ray,
        }
    }
}

fn charge(laser_query: Query<(&Laser, &Visuals)>, mut visibility_query: Query<&mut Visibility>) {
    for (laser, models) in &laser_query {
        let mut normal_visibility = visibility_query.get_mut(models.normal).unwrap();
        normal_visibility.is_visible = !laser.charging();
        let mut charging_visibility = visibility_query.get_mut(models.charging).unwrap();
        charging_visibility.is_visible = laser.charging();
    }
}

fn attack(
    laser_query: Query<(&Laser, &Visuals), Without<Player>>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (laser, visuals) in &laser_query {
        let mut visibility = visibility_query.get_mut(visuals.ray).unwrap();
        visibility.is_visible = laser.shooting();
    }
}
