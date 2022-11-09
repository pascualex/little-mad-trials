use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{palette, AppState};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Countdown::new(20.8))
            .add_startup_system(setup)
            .add_enter_system(AppState::Game, enter_game)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Game)
                    .with_system(countdown)
                    // TODO: remove
                    .with_system(instant_victory)
                    .into(),
            )
            .add_enter_system(AppState::Defeat, enter_defeat)
            .add_enter_system(AppState::Victory, enter_victory);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(MaterialMeshBundle {
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
    let text = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "20",
                TextStyle {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 24.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        })
        .insert(CountdownText)
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                min_size: Size::new(Val::Px(70.0), Val::Undefined),
                margin: UiRect::new(Val::Auto, Val::Px(10.0), Val::Px(10.0), Val::Auto),
                padding: UiRect::all(Val::Px(16.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: palette::DARK_BLACK.into(),
            ..default()
        })
        .push_children(&[text]);
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

pub struct Countdown {
    pub timer: Timer,
}

impl Countdown {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, false),
        }
    }
}

#[derive(Component)]
struct CountdownText;

fn countdown(
    mut query: Query<&mut Text, With<CountdownText>>,
    mut countdown: ResMut<Countdown>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut text = query.single_mut();
    countdown.timer.tick(time.delta());
    let remaining = countdown.timer.duration() - countdown.timer.elapsed();
    text.sections[0].value = format!("{:.0}", remaining.as_secs_f32());
    if countdown.timer.finished() {
        commands.insert_resource(NextState(AppState::Victory));
    }
}

fn instant_victory(input: Res<Input<KeyCode>>, mut commands: Commands) {
    if input.pressed(KeyCode::V) {
        commands.insert_resource(NextState(AppState::Victory));
    }
}
