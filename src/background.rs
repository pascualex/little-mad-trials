use bevy::prelude::*;

use crate::{palette, AppState};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Countdown::new(20.8))
            .add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(enter_game))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(countdown))
            .add_system_set(SystemSet::on_enter(AppState::Defeat).with_system(enter_defeat))
            .add_system_set(SystemSet::on_enter(AppState::Victory).with_system(enter_victory));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(4.0, 2.5, 0.1))),
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
    screen_ui(&mut commands, &asset_server);
}

fn screen_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let root = NodeBundle {
        style: Style {
            min_size: Size::new(Val::Px(70.0), Val::Undefined),
            margin: UiRect::new(Val::Auto, Val::Px(10.0), Val::Px(10.0), Val::Auto),
            padding: UiRect::all(Val::Px(16.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: palette::DARK_BLACK.into(),
        ..default()
    };
    let text = (
        TextBundle {
            text: Text::from_section(
                "20",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 24.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        CountdownText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(text);
    });
}

fn enter_game(mut countdown: ResMut<Countdown>) {
    countdown.timer.reset();
}

fn enter_defeat(mut query: Query<&mut Text, With<CountdownText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = "Defeat".to_string();
}

fn enter_victory(mut query: Query<&mut Text, With<CountdownText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = "Victory!".to_string();
}

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
}

impl Countdown {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

#[derive(Component)]
struct CountdownText;

fn countdown(
    mut query: Query<&mut Text, With<CountdownText>>,
    mut countdown: ResMut<Countdown>,
    time: Res<Time>,
    mut state: ResMut<State<AppState>>,
) {
    let mut text = query.single_mut();
    countdown.timer.tick(time.delta());
    let remaining = countdown.timer.duration() - countdown.timer.elapsed();
    text.sections[0].value = format!("{:.0}", remaining.as_secs_f32());
    if countdown.timer.finished() {
        state.overwrite_set(AppState::Victory).unwrap();
    }
}
