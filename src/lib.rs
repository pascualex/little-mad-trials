mod background;
mod board;
mod laser;
mod palette;
mod phases;
mod player;
mod post_processing;

use background::BackgroundPlugin;
use bevy::{
    core_pipeline::{bloom::BloomSettings, fxaa::Fxaa},
    prelude::*,
};
use post_processing::{PostProcessing, PostProcessingPlugin};

use self::{board::BoardPlugin, laser::LaserPlugin, player::PlayerPlugin};

const SHADOW_SIZE: f32 = 11.0;
const LOW_CHROMATIC_ABERRATION: f32 = 0.0;
const MEDIUM_CHROMATIC_ABERRATION: f32 = 0.001;
const HIGH_CHROMATIC_ABERRATION: f32 = 0.003;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Splash)
            .add_plugin(BackgroundPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(LaserPlugin)
            .add_plugin(PostProcessingPlugin)
            .insert_resource(Msaa { samples: 1 })
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_update(AppState::Splash).with_system(start))
            .add_system_set(SystemSet::on_update(AppState::Defeat).with_system(restart))
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(restart));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    Splash,
    Setup,
    Start,
    Game,
    Defeat,
    Victory,
    Teardown,
}

fn setup(mut commands: Commands) {
    let mut camera = commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(5.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        UiCameraConfig { show_ui: false },
        Fxaa::default(),
        PostProcessing::new(LOW_CHROMATIC_ABERRATION),
    ));
    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(BloomSettings::default());
    for i in [-6.0, 4.0] {
        for j in [-6.0, 4.0] {
            commands.spawn(DirectionalLightBundle {
                transform: Transform::from_translation(Vec3::ZERO)
                    .looking_at(Vec3::new(j, -6.0, i), Vec3::Y),
                directional_light: DirectionalLight {
                    illuminance: (i.abs() + j.abs()) * 1100.0,
                    shadows_enabled: true,
                    shadow_projection: OrthographicProjection {
                        left: -SHADOW_SIZE,
                        right: SHADOW_SIZE,
                        bottom: -SHADOW_SIZE,
                        top: SHADOW_SIZE,
                        near: -SHADOW_SIZE,
                        far: SHADOW_SIZE,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn start(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Space) {
        state.overwrite_set(AppState::Setup).unwrap();
        input.clear(); // avoids infinite loops until stageless
    }
}

fn restart(mut input: ResMut<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Space) {
        state.overwrite_set(AppState::Teardown).unwrap();
        input.clear(); // avoids infinite loops until stageless
    }
}

pub fn material_from_color(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        metallic: 0.1,
        perceptual_roughness: 0.7,
        reflectance: 0.3,
        ..default()
    }
}
