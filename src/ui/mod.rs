use std::ops::Mul;

use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Query, Res, ResMut, State, With},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    transform::components::Transform,
};

use crate::game::{GameResources, GameState};

#[derive(Component)]
pub struct EnergyBar;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ui_on_init)
            .add_systems(Update, update_energy_bar);
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

pub fn spawn_ui_on_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    // mut next_game_state: ResMut<NextState<GameState>>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
) {
    if *game_state.get() == GameState::Init {
        let scale_f = 3.;
        let sprite_size = Vec2 { x: 61., y: 22. };
        let energy_bar_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/energy-bar.png"),
            sprite_size,
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
                    custom_size: Some(Vec2::mul(sprite_size, scale_f)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}
