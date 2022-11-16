use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{
    laser::LaserMode,
    phases::{self, Phases},
    player::Player,
    post_processing::PostProcessing,
    HIGH_CHROMATIC_ABERRATION, LOW_CHROMATIC_ABERRATION,
};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(charge.after(phases::transition::<LaserMode>))
            .add_system(attack.after(phases::transition::<LaserMode>));
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
    let root = (
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 4.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.15, 0.18) * 2.0,
                metallic: 0.1,
                perceptual_roughness: 0.7,
                reflectance: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            visibility: Visibility { is_visible: false },
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    );
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

fn charge(
    laser_query: Query<(&Phases<LaserMode>, &Visuals)>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (phases, models) in &laser_query {
        let charging = matches!(phases.mode(), LaserMode::Charging | LaserMode::Shooting);
        let mut normal_visibility = visibility_query.get_mut(models.normal).unwrap();
        normal_visibility.is_visible = !charging;
        let mut charging_visibility = visibility_query.get_mut(models.charging).unwrap();
        charging_visibility.is_visible = charging;
    }
}

fn attack(
    laser_query: Query<(&Phases<LaserMode>, &Visuals), Without<Player>>,
    mut visibility_query: Query<&mut Visibility>,
    mut post_processing_query: Query<&mut PostProcessing>,
) {
    let mut any_shooting = false;
    for (phases, visuals) in &laser_query {
        let shooting = matches!(phases.mode(), LaserMode::Shooting);
        let mut visibility = visibility_query.get_mut(visuals.ray).unwrap();
        visibility.is_visible = shooting;
        any_shooting |= shooting;
    }
    let mut post_processing = post_processing_query.single_mut();
    post_processing.aberration = match any_shooting {
        false => LOW_CHROMATIC_ABERRATION,
        true => HIGH_CHROMATIC_ABERRATION,
    };
}
