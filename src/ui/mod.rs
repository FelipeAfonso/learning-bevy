use std::ops::Mul;

use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    audio::{AudioBundle, AudioSourceBundle, PlaybackMode, PlaybackSettings, Volume, VolumeLevel},
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Deref, DerefMut, Entity, Query, Res, ResMut, State, With},
    render::view::Visibility,
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};

use crate::{
    controllers::PlayerControllerState,
    game::{GameResources, GameState},
};

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

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ui_on_init)
            .add_systems(Update, update_energy_bar)
            .add_systems(Update, manage_songs)
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
    state: Res<PlayerControllerState>,
    time: Res<Time>,
) {
    for (mut sprite, mut visibility, mut timer) in &mut query {
        *visibility = Visibility::Visible;
        if state.is_boosting() {
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

pub fn update_energy_bar(
    mut query: Query<&mut TextureAtlasSprite, With<EnergyBar>>,
    game_resources: Res<GameResources>,
) {
    for mut energy_bar in &mut query {
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
                    ..Default::default()
                },
                ..Default::default()
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
                transform: Transform::from_translation(Vec3::new(-4., -260., 12.)),
                visibility: Visibility::Hidden,
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::mul(fire_sprite_size, scale_f)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}
