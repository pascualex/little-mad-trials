use std::time::Duration;

use bevy::prelude::*;

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
            .add_system_set(SystemSet::on_enter(AppState::Defeat).with_system(enter_defeat))
            .add_system_set(SystemSet::on_enter(AppState::Victory).with_system(enter_victory))
            .add_system_set(SystemSet::on_update(AppState::Victory).with_system(countdown))
            .add_system_set(SystemSet::on_enter(AppState::Teardown).with_system(enter_teardown))
            .add_system_set(
                SystemSet::on_update(AppState::Teardown)
                    .with_system(countdown)
                    .with_system(transition.after(countdown)),
            );
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
                "Uninitialized",
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

fn enter_setup(mut countdown: ResMut<Countdown>, mut query: Query<&mut Text, With<CountdownText>>) {
    countdown.reset(1.5, Some(AppState::Start));
    let mut text = query.single_mut();
    text.sections[0].value = "Setup".to_string();
}

fn enter_start(mut countdown: ResMut<Countdown>, mut query: Query<&mut Text, With<CountdownText>>) {
    countdown.reset(20.0, Some(AppState::Victory));
    let mut text = query.single_mut();
    text.sections[0].value = "Move!".to_string();
}

fn enter_defeat(mut query: Query<&mut Text, With<CountdownText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = "Defeat".to_string();
}

fn enter_victory(
    mut countdown: ResMut<Countdown>,
    mut query: Query<&mut Text, With<CountdownText>>,
) {
    countdown.reset(1.0, None);
    let mut text = query.single_mut();
    text.sections[0].value = "Victory!".to_string();
}

fn enter_teardown(
    mut countdown: ResMut<Countdown>,
    mut query: Query<&mut Text, With<CountdownText>>,
) {
    countdown.reset(1.5, Some(AppState::Setup));
    let mut text = query.single_mut();
    text.sections[0].value = "Teardown".to_string();
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
pub fn countdown(mut countdown: ResMut<Countdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
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
