use crate::{
    controllers::PlayerControllerState,
    game::{GameState, MOVE_SPEED, SPAWN_TIMER, SPRINGINT_SPEED},
};
use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use rand::Rng;
use std::time::Duration;
#[derive(Component)]
pub struct GameEntity;
#[derive(Component)]
pub struct PlayerEntity;
#[derive(Component)]
pub struct PlayerAttached;
pub enum EnemyType {
    FLY,
    MOSQUITO,
    // FROG,
}
#[derive(Component)]
pub struct EnemyEntity {
    revert_direction: bool,
    pub enemy_type: EnemyType,
    timer: f32,
}
#[derive(Component)]
pub struct Background;
#[derive(Resource)]
struct EnemySpawner {
    timer: Timer,
}
#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, spawn_entities_on_init)
            .add_systems(Update, move_player)
            .add_systems(Update, move_web)
            .add_systems(Update, animate_sprite)
            .add_systems(Update, spawn_enemies)
            .add_systems(Update, move_enemies)
            .add_systems(Update, despawn_game_entities_on_game_over);
    }
}

pub fn move_web(
    mut query: Query<&mut Transform, With<PlayerAttached>>,
    player_query: Query<&mut Transform, (With<PlayerEntity>, Without<PlayerAttached>)>,
) {
    for player in &mut player_query.iter() {
        for mut web in &mut query {
            web.translation.x = player.translation.x;
            web.translation.y = player.translation.y + 500.;
        }
    }
}

pub fn move_player(
    time: Res<Time>,
    state: Res<PlayerControllerState>,
    mut query: Query<&mut Transform, (With<PlayerEntity>, Without<PlayerAttached>)>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::Active {
        let st = state.get_state();
        let speed: f32 = if state.is_boosting() {
            SPRINGINT_SPEED
        } else {
            MOVE_SPEED
        };
        for mut player in &mut query {
            // check for collisions on x axis
            if player.translation.x < -618. {
                player.translation.x = -618.;
            } else if player.translation.x > 618. {
                player.translation.x = 618.;
            } else {
                player.translation.x += st.0 * speed * time.delta_seconds();
            }
            // check on the y axis
            if player.translation.y < -328. {
                player.translation.y = -328.;
            } else if player.translation.y > 328. {
                player.translation.y = 328.;
            } else {
                player.translation.y += st.1 * speed * time.delta_seconds();
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    game_state: Res<State<GameState>>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    if *game_state.get() == GameState::Active {
        for (indices, mut timer, mut sprite) in &mut query {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
}

fn despawn_game_entities_on_game_over(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<GameEntity>>,
) {
    if *game_state.get() == GameState::GameOver {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        next_game_state.set(GameState::StartMenu);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<EnemySpawner>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state: Res<State<GameState>>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
) {
    if *game_state.get() == GameState::Active {
        config.timer.tick(time.delta());
        if config.timer.finished() {
            let mut rng = rand::thread_rng();
            let window = window_query.get_single().unwrap();
            let enemy_type = match rand::random::<bool>() {
                true => EnemyType::FLY,
                false => EnemyType::MOSQUITO,
            };
            let size = match enemy_type {
                EnemyType::FLY => Vec2 { x: 16., y: 16. },
                EnemyType::MOSQUITO => Vec2 { x: 16., y: 10. },
            };
            let sprite = match enemy_type {
                EnemyType::FLY => "sprites/fly.png",
                EnemyType::MOSQUITO => "sprites/mosquito.png",
            };
            let animation_indices = AnimationIndices { first: 0, last: 1 };
            let animation_timer = match enemy_type {
                EnemyType::FLY => AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
                EnemyType::MOSQUITO => {
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
                }
            };
            let height = window.height() - size.y;
            let width = window.width() - size.x;
            let half_height = height / 2.;
            let half_width = width / 2.;
            let revert_direction = rand::random::<bool>();
            let y: f32 = (rng.gen::<f32>() * height) - half_height;
            let x: f32 = if revert_direction {
                half_width - size.x
            } else {
                (half_width * -1.) + size.x
            };

            let enemy_atlas = TextureAtlas::from_grid(
                asset_server.load(sprite),
                size,
                2,
                1,
                Some(Vec2::splat(1.)),
                None,
            );
            let enemy_atlas_handle = texture_atlasses.add(enemy_atlas);
            commands.spawn((
                GameEntity,
                EnemyEntity {
                    revert_direction,
                    enemy_type,
                    timer: 0.,
                },
                SpriteSheetBundle {
                    texture_atlas: enemy_atlas_handle,
                    transform: Transform {
                        translation: Vec3::from((x, y, 2.)),
                        ..default()
                    },
                    sprite: TextureAtlasSprite {
                        index: animation_indices.first,
                        custom_size: Some(size * 2.),
                        flip_x: revert_direction,
                        ..default()
                    },
                    ..default()
                },
                animation_indices,
                animation_timer,
            ));
        }
    }
}

fn move_enemies(
    mut query: Query<(&mut Transform, &mut EnemyEntity)>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::Active {
        for mut enemy in &mut query {
            enemy.1.timer += time.delta_seconds();
            match enemy.1.enemy_type {
                EnemyType::FLY => {
                    let movement: f32 = time.delta_seconds() * 64.;
                    enemy.0.translation.x += if enemy.1.revert_direction {
                        movement * -1.
                    } else {
                        movement
                    };
                    // vertical oscillation
                    enemy.0.translation.y += movement * (enemy.1.timer * 3.).sin();
                }
                EnemyType::MOSQUITO => {
                    let movement: f32 = time.delta_seconds() * 256.;
                    enemy.0.translation.x += if enemy.1.revert_direction {
                        movement * -1.
                    } else {
                        movement
                    }
                }
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(EnemySpawner {
        timer: Timer::new(Duration::from_secs_f32(SPAWN_TIMER), TimerMode::Repeating),
    });
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "State: Active\n",
                TextStyle {
                    font_size: 20.,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/geist.ttf"),
                },
            ),
            TextSection::new(
                "Score: 0",
                TextStyle {
                    font_size: 20.,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/geist.ttf"),
                },
            ),
            TextSection::new(
                "Energy: 100%",
                TextStyle {
                    font_size: 20.,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/geist.ttf"),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        }),
    );
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1280.,
                height: 720.,
            },
            far: 1000.,
            near: -1000.,
            ..default()
        },
        ..default()
    });
}

pub fn spawn_entities_on_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
) {
    if *game_state.get() == GameState::Init {
        commands.spawn((
            Background,
            SpriteBundle {
                texture: asset_server.load("sprites/bgblur.png"),
                transform: Transform {
                    translation: Vec3::from((0., 0., 0.)),
                    scale: Vec3::from((2., 2., 1.)),
                    ..default()
                },
                ..default()
            },
        ));
        let spider_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/spooder.png"),
            Vec2 { x: 32., y: 32. },
            2,
            1,
            None,
            None,
        );
        let spider_atlas_handle = texture_atlasses.add(spider_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 1 };
        commands.spawn((
            PlayerEntity,
            GameEntity,
            SpriteSheetBundle {
                texture_atlas: spider_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    custom_size: Some(Vec2::splat(64.)),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        ));
        commands.spawn((
            PlayerEntity,
            GameEntity,
            PlayerAttached,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(3.0, 1000.0)),
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::from((0., 500., 1.)),
                    ..default()
                },
                ..default()
            },
        ));
        next_game_state.set(GameState::Active);
    }
}
