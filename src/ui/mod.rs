use crate::{
    controllers::PlayerControllerState,
    entities::Background,
    game::{GameResources, GameState},
};
use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    audio::{AudioBundle, AudioSourceBundle, PlaybackMode, PlaybackSettings, Volume, VolumeLevel},
    input::mouse::MouseButtonInput,
    math::{Vec2, Vec3},
    prelude::{
        default, Commands, Component, Deref, DerefMut, Entity, EventReader, NextState, Query, Res,
        ResMut, State, With,
    },
    render::view::Visibility,
    sprite::{
        collide_aabb::collide, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite,
    },
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};
use std::ops::Mul;
#[derive(Component)]
pub struct Song {
    title: String,
}
#[derive(Component)]
pub struct EnergyBarFire;
#[derive(Component)]
pub struct EnergyBar;
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
#[derive(Component)]
pub struct StartMenuUI;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ui_on_init)
            .add_systems(Update, update_energy_bar)
            .add_systems(Update, manage_songs)
            .add_systems(Update, manage_start_button)
            .add_systems(Update, show_start_menu_ui)
            .add_systems(Update, update_energy_bar_fire);
    }
}

pub fn update_energy_bar_fire(
    mut query: Query<
        (
            &mut TextureAtlasSprite,
            &mut Visibility,
            &mut AnimationTimer,
        ),
        With<EnergyBarFire>,
    >,
    controller_state: Res<PlayerControllerState>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
) {
    let state = *game_state.get();
    for (mut sprite, mut visibility, mut timer) in &mut query {
        match state {
            GameState::StartMenu | GameState::GameOver => {
                *visibility = Visibility::Hidden;
            }
            _ => {
                *visibility = Visibility::Visible;
                if controller_state.is_boosting() {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        sprite.index = if sprite.index == 3 {
                            0
                        } else {
                            sprite.index + 1
                        };
                    }
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn update_energy_bar(
    mut query: Query<(&mut TextureAtlasSprite, &mut Visibility), With<EnergyBar>>,
    game_resources: Res<GameResources>,
    game_state: Res<State<GameState>>,
) {
    let state = *game_state.get();
    for (mut energy_bar, mut visibility) in &mut query {
        match state {
            GameState::StartMenu | GameState::GameOver => {
                *visibility = Visibility::Hidden;
            }
            _ => {
                *visibility = Visibility::Visible;
                let index = match game_resources.energy {
                    0.0..=0.25 => 1,
                    0.25..=0.4 => 2,
                    0.4..=0.55 => 3,
                    0.55..=0.7 => 4,
                    0.7..=0.85 => 5,
                    0.85..=1.0 => 6,
                    _ => 0,
                };
                energy_bar.index = index;
            }
        }
    }
}

fn new_song_tuple(name: &str, asset_server: &Res<AssetServer>) -> (Song, AudioSourceBundle) {
    (
        Song {
            title: String::from(name),
        },
        AudioBundle {
            source: asset_server.load(format!("sound/{}.mp3", &name)),
            settings: PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(if name == "theme" { 1. } else { 0.7 })),
                mode: PlaybackMode::Loop,
                speed: 1.,
                spatial: false,
                paused: false,
            },
        },
    )
}

pub fn manage_songs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    mut query: Query<(Entity, &Song)>,
) {
    let state = *game_state.get();
    if query.is_empty() && state == GameState::StartMenu {
        commands.spawn(new_song_tuple("theme", &asset_server));
    }
    for (entity, song) in &mut query {
        match state {
            GameState::StartMenu => {
                if song.title != String::from("theme") {
                    commands.entity(entity).despawn();
                    commands.spawn(new_song_tuple("theme", &asset_server));
                }
            }
            _ => {
                if song.title != String::from("in_the_jungle") {
                    commands.entity(entity).despawn();
                    commands.spawn(new_song_tuple("in_the_jungle", &asset_server));
                }
            }
        }
    }
}

pub fn manage_start_button(
    mut interaction_query: Query<(&mut TextureAtlasSprite, &Transform), With<StartMenuUI>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut click_events: EventReader<MouseButtonInput>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_resources: ResMut<GameResources>,
) {
    let window = window_query.get_single().unwrap();
    match window.cursor_position() {
        Some(cursor_pos) => {
            for (mut sprite, transform) in &mut interaction_query {
                let cursor_pos = Vec3::from_array([cursor_pos[0], cursor_pos[1], 0.]);
                let cursor_size = Vec2::splat(1.);
                let el_pos = Vec3::from_array(transform.translation.into());
                let spr_copy = sprite.clone();
                let el_size = Vec2::from_array(spr_copy.custom_size.unwrap().into());
                let screen_center =
                    Vec3::from_array([window.width() / 2., window.height() / 2., 0.]);
                let normalized_el_pos = screen_center - el_pos;
                let final_el_pos =
                    Vec3::from_array([normalized_el_pos[0], normalized_el_pos[1], 0.]);
                match collide(cursor_pos, cursor_size, final_el_pos, el_size) {
                    Some(_col) => {
                        sprite.index = 1;
                        if click_events.read().next().is_some() {
                            if *game_state.get() == GameState::StartMenu {
                                next_game_state.set(GameState::Init);
                                game_resources.score = 0;
                                game_resources.time = 0.;
                                game_resources.energy = 1.;
                            }
                        }
                    }
                    _ => {
                        sprite.index = 0;
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn show_start_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    query: Query<Entity, With<StartMenuUI>>,
) {
    if *game_state.get() == GameState::StartMenu {
        if !query.is_empty() {
            return;
        }
        let play_button_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/play-button.png"),
            Vec2 { x: 28., y: 17. },
            1,
            2,
            None,
            None,
        );
        let play_button_atlas_handle = texture_atlasses.add(play_button_atlas);
        commands.spawn((
            StartMenuUI,
            SpriteSheetBundle {
                texture_atlas: play_button_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., -260., 15.)),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::from([224., 136.])),
                    ..default()
                },
                ..default()
            },
        ));
        commands.spawn((
            Background,
            StartMenuUI,
            SpriteBundle {
                texture: asset_server.load("sprites/bg.png"),
                transform: Transform {
                    translation: Vec3::from((0., 0., 0.)),
                    scale: Vec3::from((2., 2., 1.)),
                    ..default()
                },
                ..default()
            },
        ));
    } else {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_ui_on_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    // mut next_game_state: ResMut<NextState<GameState>>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
) {
    if *game_state.get() == GameState::Init {
        let scale_f = 3.;
        let energy_bar_sprite_size = Vec2 { x: 61., y: 22. };
        let energy_bar_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/energy-bar.png"),
            energy_bar_sprite_size,
            1,
            7,
            Some(Vec2::splat(0.)),
            // None,
            None,
        );
        let energy_bar_atlas_handle = texture_atlasses.add(energy_bar_atlas);
        commands.spawn((
            EnergyBar,
            SpriteSheetBundle {
                texture_atlas: energy_bar_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., -300., 15.)),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::mul(energy_bar_sprite_size, scale_f)),
                    ..default()
                },
                ..default()
            },
        ));
        let fire_sprite_size = Vec2 { x: 69., y: 27. };
        let fire_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/energy-bar-fire.png"),
            fire_sprite_size,
            1,
            4,
            Some(Vec2::splat(0.)),
            // None,
            None,
        );
        let fire_atlas_handle = texture_atlasses.add(fire_atlas);
        commands.spawn((
            EnergyBarFire,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            SpriteSheetBundle {
                texture_atlas: fire_atlas_handle,
                visibility: Visibility::Hidden,
                transform: Transform::from_translation(Vec3::new(-4., -260., 12.)),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::mul(fire_sprite_size, scale_f)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}
