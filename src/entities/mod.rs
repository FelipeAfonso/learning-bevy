use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use rand::Rng;

use crate::{controllers::PlayerControllerState, game::GameState};

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
        app.add_startup_system(setup)
            .add_system(move_player)
            .add_system(spawn_enemies)
            .add_system(move_enemies);
    }
}

pub fn move_player(
    time: Res<Time>,
    state: Res<PlayerControllerState>,
    mut query: Query<&mut Transform, With<PlayerEntity>>,
    game_state: Res<State<GameState>>,
) {
    if game_state.0 == GameState::Active {
        let st = state.get_state();
        //println!(" -- x: {} -- y: {} --", st.0, st.1);
        let speed: f32 = if state.is_boosting() { 256.0 } else { 128.0 };

        for mut player in &mut query {
            player.translation.x += st.0 * speed * time.delta_seconds();
            player.translation.y += st.1 * speed * time.delta_seconds();
            println!("x: {}  y: {}", player.translation.x, player.translation.y)
        }
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
    if game_state.0 == GameState::Active {
        config.timer.tick(time.delta());

        if config.timer.finished() {
            let mut rng = rand::thread_rng();

            let window = window_query.get_single().unwrap();
            let half_height = window.height() / 2.;
            let half_width = window.width() / 2.;

            let revert_direction = rand::random::<bool>();

            let y: f32 = (rng.gen::<f32>() * window.height()) - half_height;
            commands.spawn((
                EnemyEntity { revert_direction },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(30.0, 20.0)),
                        flip_x: revert_direction,
                        ..default()
                    },
                    texture: asset_server.load("sprites/enemy-1.png"),
                    transform: Transform {
                        translation: Vec3::from((
                            if revert_direction {
                                half_width
                            } else {
                                half_width * -1.
                            },
                            y,
                            0.,
                        )),
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
    if game_state.0 == GameState::Active {
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

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
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
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            texture: asset_server.load("sprites/orange-spider.png"),
            ..default()
        },
    ));

    commands.spawn(
        TextBundle::from_section(
            "State: Active",
            TextStyle {
                font_size: 20.,
                color: Color::WHITE,
                font: asset_server.load("fonts/geist.ttf"),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.),
                right: Val::Px(5.),
                ..default()
            },
            ..default()
        }),
    );

    commands.spawn((
        PlayerEntity,
        PlayerAttached,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(5.0, 1000.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::from((0., 500., 0.)),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn(Camera2dBundle { ..default() });

    commands.insert_resource(EnemySpawner {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });
}
