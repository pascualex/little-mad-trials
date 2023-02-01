use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    pbr::NotShadowReceiver,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
    },
};

use crate::{
    background::{self, Countdown},
    material_from_color, palette, AppState,
};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Countdown::new())
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(show_countdown.after(background::countdown)),
            )
            .add_system(show_screen_elements)
            .add_system(flip)
            .add_system(spin);
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

    // screen render texture
    let size = Extent3d {
        width: 512,
        height: 350,
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
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let handle = images.add(image);

    // screen camera
    commands.spawn((Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::NONE),
        },
        camera: Camera {
            target: RenderTarget::Image(handle.clone()),
            order: -1,
            ..default()
        },
        ..default()
    },));

    // screen
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(4.25, 2.50, 0.1))),
            material: materials.add(material_from_color(palette::DARK_BLACK * 0.85)),
            transform: Transform::from_xyz(0.0, 3.75, -6.0),
            ..default()
        },
        NotShadowReceiver,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 40.0, 0.1))),
            material: materials.add(material_from_color(palette::DARK_BLACK * 0.85)),
            transform: Transform::from_xyz(0.0, 22.5, -6.2),
            ..default()
        },
        NotShadowReceiver,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(4.0, 2.25)))),
            material: materials.add(StandardMaterial {
                ..material_from_color(palette::LIGHT_WHITE * 1.22)
            }),
            transform: Transform::from_xyz(0.0, 3.75, -5.94),
            ..default()
        },
        NotShadowReceiver,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(4.0, 2.25)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(handle),
                alpha_mode: AlphaMode::Blend,
                ..material_from_color(Color::WHITE)
            }),
            transform: Transform::from_xyz(0.0, 3.75, -5.93),
            ..default()
        },
        NotShadowReceiver,
    ));
}

fn screen_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let root = NodeBundle {
        style: Style {
            size: Size::new(Val::Px(512.0), Val::Px(350.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };
    let splash_top_text = (
        TextBundle {
            text: Text::from_section(
                " ",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(5.0), Val::Undefined),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Splash),
    );
    let splash_text = (
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Little Mad\nTrials",
                    TextStyle {
                        font: asset_server.load("fonts/roboto_bold.ttf"),
                        font_size: 100.0,
                        color: Color::BLACK,
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Splash),
    );
    let splash_bottom_text = (
        TextBundle {
            text: Text::from_section(
                "[space] to start",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 75.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Undefined, Val::Px(5.0)),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Splash),
    );
    let start_top_text = (
        TextBundle {
            text: Text::from_section(
                " ",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(5.0), Val::Undefined),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Start),
    );
    let dodge_text = (
        TextBundle {
            text: Text::from_section(
                "Dodge!",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 124.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Start),
    );
    let start_bottom_text = (
        TextBundle {
            text: Text::from_section(
                "[arrows] to move",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 75.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Undefined, Val::Px(5.0)),
                ..default()
            },
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
                    color: Color::BLACK,
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
        Spin::new(1.2),
    );
    let defeat_top_text = (
        TextBundle {
            text: Text::from_section(
                " ",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(5.0), Val::Undefined),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Defeat),
    );
    let skull = (
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(170.0), Val::Px(180.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            image: asset_server.load("sprites/skull.png").into(),
            ..default()
        },
        ScreenElement::new(AppState::Defeat),
    );
    let defeat_bottom_text = (
        TextBundle {
            text: Text::from_section(
                "[space] to retry",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 75.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Undefined, Val::Px(5.0)),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Defeat),
    );
    let victory_top_text = (
        TextBundle {
            text: Text::from_section(
                "Victory!",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 75.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(5.0), Val::Undefined),
                ..default()
            },
            ..default()
        },
        ScreenElement::new(AppState::Victory),
    );
    let popper = (
        ImageBundle {
            style: Style {
                size: Size::new(Val::Px(180.0), Val::Px(180.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            image: asset_server.load("sprites/popper.png").into(),
            ..default()
        },
        ScreenElement::new(AppState::Victory),
        Flip::new(0.8),
    );
    let victory_bottom_text = (
        TextBundle {
            text: Text::from_section(
                "[space] to replay",
                TextStyle {
                    font: asset_server.load("fonts/roboto_bold.ttf"),
                    font_size: 75.0,
                    color: Color::BLACK,
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Undefined, Val::Px(5.0)),
                ..default()
            },
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
        Flip::new(0.5),
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(splash_top_text);
        builder.spawn(splash_text);
        builder.spawn(splash_bottom_text);
        builder.spawn(start_top_text);
        builder.spawn(dodge_text);
        builder.spawn(start_bottom_text);
        builder.spawn(countdown_text);
        builder.spawn(gear);
        builder.spawn(defeat_top_text);
        builder.spawn(skull);
        builder.spawn(defeat_bottom_text);
        builder.spawn(victory_top_text);
        builder.spawn(popper);
        builder.spawn(victory_bottom_text);
        builder.spawn(broom);
    });
}

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct ScreenElement {
    state: AppState,
}

#[derive(Component)]
struct Flip {
    timer: Timer,
}

impl Flip {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
struct Spin {
    timer: Timer,
}

impl Spin {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

impl ScreenElement {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
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

fn flip(mut query: Query<(&mut Flip, &mut UiImage)>, time: Res<Time>) {
    for (mut flip, mut ui_image) in &mut query {
        flip.timer.tick(time.delta());
        ui_image.flip_x = flip.timer.percent() >= 0.5;
    }
}

fn spin(mut query: Query<(&mut Spin, &mut Transform)>, time: Res<Time>) {
    for (mut spin, mut transform) in &mut query {
        spin.timer.tick(time.delta());
        transform.rotation = Quat::from_rotation_z(2.0 * PI * spin.timer.percent());
    }
}

fn show_countdown(mut query: Query<&mut Text, With<CountdownText>>, countdown: Res<Countdown>) {
    let mut text = query.single_mut();
    let remaining = countdown.timer.duration() - countdown.timer.elapsed();
    text.sections[0].value = format!("{:.1}", remaining.as_secs_f32());
}
