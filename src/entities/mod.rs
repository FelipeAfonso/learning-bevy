use crate::{
    controllers::PlayerControllerState,
    game::{
        GameState, ENEMY_SPRITE_HEIGHT, ENEMY_SPRITE_WIDTH, MOVE_SPEED, SCREEN_OFFSET, SPAWN_TIMER,
        SPRINGINT_SPEED,
    },
};
use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
pub struct GameEntity;
#[derive(Component)]
pub struct PlayerEntity;
#[derive(Component)]
pub struct PlayerAttached;
#[derive(Component)]
pub struct EnemyEntity {
    revert_direction: bool,
}
#[derive(Component)]
pub struct Background;

#[derive(Resource)]
struct EnemySpawner {
    timer: Timer,
}

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, spawn_entities_on_init)
            .add_systems(Update, move_player)
            .add_systems(Update, spawn_enemies)
            .add_systems(Update, move_enemies)
            .add_systems(Update, despawn_game_entities_on_game_over);
    }
}

pub fn move_player(
    time: Res<Time>,
    state: Res<PlayerControllerState>,
    mut query: Query<&mut Transform, With<PlayerEntity>>,
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
            player.translation.x += st.0 * speed * time.delta_seconds();
            player.translation.y += st.1 * speed * time.delta_seconds();
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
        next_game_state.set(GameState::Init);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<EnemySpawner>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::Active {
        config.timer.tick(time.delta());
        if config.timer.finished() {
            let mut rng = rand::thread_rng();
            let window = window_query.get_single().unwrap();
            println!("w: {} - h: {}", window.width(), window.height());
            let height = window.height() - SCREEN_OFFSET.y;
            let width = window.width() - SCREEN_OFFSET.x;
            let half_height = height / 2.;
            let half_width = width / 2.;
            let revert_direction = rand::random::<bool>();
            let y: f32 = (rng.gen::<f32>() * height) - half_height;
            let x: f32 = if revert_direction {
                half_width - SCREEN_OFFSET.x
            } else {
                (half_width * -1.) + SCREEN_OFFSET.x
            };

            commands.spawn((
                GameEntity,
                EnemyEntity { revert_direction },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(ENEMY_SPRITE_WIDTH, ENEMY_SPRITE_HEIGHT)),
                        flip_x: revert_direction,
                        ..default()
                    },
                    texture: asset_server.load("sprites/enemy-1.png"),
                    transform: Transform {
                        translation: Vec3::from((x, y, 0.)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

fn move_enemies(
    mut query: Query<(&mut Transform, &EnemyEntity)>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::Active {
        let movement: f32 = time.delta_seconds() * 64.;
        for mut enemy in &mut query {
            enemy.0.translation.x += if enemy.1.revert_direction {
                movement * -1.
            } else {
                movement
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
    commands.spawn(Camera2dBundle { ..default() });
    commands.spawn(AudioBundle {
        source: asset_server.load("sound/theme.ogg"),
        settings: PlaybackSettings {
            volume: Volume::Relative(VolumeLevel::new(1.)),
            mode: PlaybackMode::Loop,
            speed: 1.,
            spatial: false,
            paused: false,
        },
    });
}

pub fn spawn_entities_on_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if *game_state.get() == GameState::Init {
        let window = window_query.get_single().unwrap();
        let width = window.width();
        let height = window.height();

        commands.spawn((
            Background,
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::default().with_scale(Vec3::from([width, height, 1.])),
                material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
                ..default()
            },
        ));

        commands.spawn((
            PlayerEntity,
            GameEntity,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                texture: asset_server.load("sprites/orange-spider.png"),
                ..default()
            },
        ));

        commands.spawn((
            PlayerEntity,
            GameEntity,
            PlayerAttached,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5.0, 1000.0)),
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::from((0., 500., 0.)),
                    ..default()
                },
                ..default()
            },
        ));

        next_game_state.set(GameState::Active);
    }
}
