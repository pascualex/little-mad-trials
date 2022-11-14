use bevy::prelude::*;

use crate::{
    laser::{self, Laser, Mode},
    palette,
    player::Player,
};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(charge.after(laser::mode))
            .add_system(attack.after(laser::mode));
    }
}

pub fn turrets_blueprint(
    commands: &mut Commands,
    color: Color,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let root = (TransformBundle::default(), VisibilityBundle::default());
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
        .spawn(root)
        .with_children(|builder| {
            builder.spawn(top);
            builder.spawn(bottom);
        })
        .id()
}

pub fn ray_blueprint(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let root = MaterialMeshBundle {
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
    };
    commands.spawn(root).id()
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
        normal_visibility.is_visible = !matches!(laser.mode(), Mode::Charging | Mode::Shooting);
        let mut charging_visibility = visibility_query.get_mut(models.charging).unwrap();
        charging_visibility.is_visible = matches!(laser.mode(), Mode::Charging | Mode::Shooting);
    }
}

fn attack(
    laser_query: Query<(&Laser, &Visuals), Without<Player>>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (laser, visuals) in &laser_query {
        let mut visibility = visibility_query.get_mut(visuals.ray).unwrap();
        visibility.is_visible = matches!(laser.mode(), Mode::Shooting);
    }
}
