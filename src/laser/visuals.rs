use std::f32::consts::PI;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{
    background::Countdown,
    laser::{Laser, LaserMode},
    material_from_color,
    phases::{self, Phases},
    player::Player,
    post_processing::PostProcessing,
    HIGH_CHROMATIC_ABERRATION, LOW_CHROMATIC_ABERRATION, MEDIUM_CHROMATIC_ABERRATION,
};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(charge.after(phases::transition::<LaserMode>))
            .add_system(attack.after(phases::transition::<LaserMode>))
            .add_system(attack_sound.after(phases::transition::<LaserMode>));
    }
}

pub fn turrets_blueprint(
    mobile: bool,
    commands: &mut Commands,
    color: Color,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let root = (TransformBundle::default(), VisibilityBundle::default());
    let top = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
        material: materials.add(material_from_color(color)),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    };
    let bottom = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.3, 0.3, 0.6))),
        material: materials.add(material_from_color(color)),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    };
    let top_rail = (MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.6, 0.15, 0.3))),
        material: materials.add(material_from_color(color)),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    },);
    let bottom_rail = (MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.6, 0.15, 0.3))),
        material: materials.add(material_from_color(color)),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    },);
    commands
        .spawn(root)
        .with_children(|builder| {
            builder.spawn(top);
            builder.spawn(bottom);
            if mobile {
                builder.spawn(top_rail);
                builder.spawn(bottom_rail);
            }
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
            mesh: meshes.add(Mesh::from(shape::Capsule {
                depth: 4.0,
                radius: 0.075,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                emissive: Color::rgb(1.0, 0.02, 0.03) * 5.0,
                ..material_from_color(Color::rgb(1.0, 0.1, 0.12))
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
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
    pub fast: Entity,
    pub charging: Entity,
    pub ray: Entity,
}

impl Visuals {
    pub fn new(normal: Entity, fast: Entity, charging: Entity, ray: Entity) -> Self {
        Self {
            normal,
            fast,
            charging,
            ray,
        }
    }
}

fn charge(
    laser_query: Query<(&Laser, &Phases<LaserMode>, &Visuals)>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (laser, phases, models) in &laser_query {
        let charging = matches!(phases.mode(), LaserMode::Charging | LaserMode::Attacking);
        let mut normal_visibility = visibility_query.get_mut(models.normal).unwrap();
        normal_visibility.is_visible = !charging && !laser.fast;
        let mut fast_visibility = visibility_query.get_mut(models.fast).unwrap();
        fast_visibility.is_visible = !charging && laser.fast;
        let mut charging_visibility = visibility_query.get_mut(models.charging).unwrap();
        charging_visibility.is_visible = charging;
    }
}

fn attack(
    laser_query: Query<(&Phases<LaserMode>, &Visuals), Without<Player>>,
    mut visibility_query: Query<&mut Visibility>,
    mut post_processing_query: Query<&mut PostProcessing>,
    countdown: Res<Countdown>,
) {
    let mut shooters = 0;
    for (phases, visuals) in &laser_query {
        let shooting = matches!(phases.mode(), LaserMode::Attacking);
        let mut visibility = visibility_query.get_mut(visuals.ray).unwrap();
        visibility.is_visible = shooting;
        shooters += shooting as i32;
    }
    let mut post_processing = post_processing_query.single_mut();
    post_processing.aberration = match shooters > 0 {
        true => match countdown.timer.elapsed_secs() >= 16.0 && shooters >= 3 {
            true => HIGH_CHROMATIC_ABERRATION,
            false => MEDIUM_CHROMATIC_ABERRATION,
        },
        false => LOW_CHROMATIC_ABERRATION,
    };
}

pub fn attack_sound(
    query: Query<(&Laser, &Phases<LaserMode>)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (laser, phases) in &query {
        if laser.mobile && phases.just_transitioned {
            let (path, volume) = match phases.mode() {
                LaserMode::Charging => ("sounds/charge.ogg", 0.1),
                LaserMode::Attacking => ("sounds/attack.ogg", 0.2),
                LaserMode::Ready => continue,
            };
            let sound = asset_server.load(path);
            audio.play_with_settings(sound, PlaybackSettings::ONCE.with_volume(volume));
        }
    }
}
