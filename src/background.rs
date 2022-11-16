use std::time::Duration;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
    },
};

use crate::{palette, AppState};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Countdown::new())
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(enter_setup))
            .add_system_set(
                SystemSet::on_update(AppState::Setup)
                    .with_system(countdown)
                    .with_system(transition.after(countdown)),
            )
            .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(countdown)
                    .with_system(show_countdown.after(countdown))
                    .with_system(transition.after(countdown)),
            )
            .add_system_set(SystemSet::on_enter(AppState::Victory).with_system(enter_victory))
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(countdown))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system_set(
                SystemSet::on_update(AppState::Teardown)
                    .with_system(countdown)
                    .with_system(transition.after(countdown)),
            )
            .add_system(show_screen_elements);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    screen_ui(&mut commands, &asset_server);

    let size = Extent3d {
        width: 512,
        height: 288,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image.resize(size);
    let handle = images.add(image);

    commands.spawn((Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::NONE),
        },
        camera: Camera {
            target: RenderTarget::Image(handle.clone()),
            priority: -1,
            ..default()
        },
        ..default()
    },));

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(4.25, 2.50, 0.1))),
        material: materials.add(StandardMaterial {
            base_color: palette::DARK_BLACK,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 4.0, -7.0),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(4.0, 2.25)))),
        material: materials.add(StandardMaterial {
            base_color: palette::LIGHT_WHITE * 1.8,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 4.0, -6.94),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(4.0, 2.25)))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(handle),
            metallic: 0.1,
            perceptual_roughness: 0.7,
            reflectance: 0.3,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 4.0, -6.93),
        ..default()
    });
}

fn screen_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let root = (NodeBundle {
        style: Style {
            size: Size::new(Val::Px(512.0), Val::Px(288.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    },);
    let dodge_text = (
        TextBundle {
            text: Text::from_section(
                "Dodge!",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 124.0,
                    color: palette::DARK_BLACK,
                },
            ),
            ..default()
        },
        ScreenElement::new(AppState::Start),
    );
    let countdown_text = (
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 124.0,
                    color: palette::DARK_BLACK,
                },
            ),
            ..default()
        },
        ScreenElement::new(AppState::Game),
        CountdownText,
    );
    let gear = (
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(170.0), Val::Px(170.0)),
                ..default()
            },
            image: asset_server.load("sprites/gear.png").into(),
            ..default()
        },
        ScreenElement::new(AppState::Setup),
    );
    let skull = (
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(170.0), Val::Px(180.0)),
                ..default()
            },
            image: asset_server.load("sprites/skull.png").into(),
            ..default()
        },
        ScreenElement::new(AppState::Defeat),
    );
    let popper = (
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(180.0), Val::Px(180.0)),
                ..default()
            },
            image: asset_server.load("sprites/popper.png").into(),
            ..default()
        },
        ScreenElement::new(AppState::Victory),
    );
    let broom = (
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(170.0), Val::Px(170.0)),
                ..default()
            },
            image: asset_server.load("sprites/broom.png").into(),
            ..default()
        },
        ScreenElement::new(AppState::Teardown),
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(dodge_text);
        builder.spawn(countdown_text);
        builder.spawn(gear);
        builder.spawn(skull);
        builder.spawn(popper);
        builder.spawn(broom);
    });
}

fn enter_setup(mut countdown: ResMut<Countdown>) {
    countdown.reset(1.5, Some(AppState::Start));
}

fn enter_start(mut countdown: ResMut<Countdown>) {
    countdown.reset(20.0, Some(AppState::Victory));
}

fn enter_victory(mut countdown: ResMut<Countdown>) {
    countdown.reset(1.0, None);
}

fn enter_teardown(mut countdown: ResMut<Countdown>) {
    countdown.reset(1.5, Some(AppState::Setup));
}

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
    pub transition: Option<AppState>,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            transition: None,
        }
    }

    pub fn reset(&mut self, seconds: f32, transition: Option<AppState>) {
        self.timer.set_duration(Duration::from_secs_f32(seconds));
        self.timer.reset();
        self.transition = transition;
    }
}

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct ScreenElement {
    state: AppState,
}

impl ScreenElement {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

pub fn countdown(mut countdown: ResMut<Countdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
}

fn show_screen_elements(
    mut query: Query<(&ScreenElement, &mut Style)>,
    state: Res<State<AppState>>,
) {
    for (element, mut style) in &mut query {
        style.display = match element.state == *state.current() {
            true => Display::Flex,
            false => Display::None,
        };
    }
}

fn show_countdown(mut query: Query<&mut Text, With<CountdownText>>, countdown: Res<Countdown>) {
    let mut text = query.single_mut();
    let remaining = countdown.timer.duration() - countdown.timer.elapsed();
    text.sections[0].value = format!("{:.1}", remaining.as_secs_f32());
}

fn transition(countdown: Res<Countdown>, mut state: ResMut<State<AppState>>) {
    let Some(transition) = countdown.transition else {
        return;
    };
    if countdown.timer.finished() {
        state.overwrite_set(transition).unwrap();
    }
}
