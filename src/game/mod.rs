use crate::{
    controllers::PlayerControllerState,
    entities::{EnemyEntity, PlayerAttached, PlayerEntity},
};
use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
pub const IDLE_ENERGY_BURNING_RATE: f32 = 0.075;
pub const MOVING_ENERGY_BURNING_RATE: f32 = 0.125;
pub const SPRINTING_ENERGY_BURNING_RATE: f32 = 0.30;
pub const SPRINGINT_SPEED: f32 = 256.0;
pub const MOVE_SPEED: f32 = 128.0;
pub const SPAWN_TIMER: f32 = 0.8;
pub const ENEMY_SPRITE_HEIGHT: f32 = 20.;
pub const ENEMY_SPRITE_WIDTH: f32 = 30.;
pub struct ScreenOffset {
    pub x: f32,
    pub y: f32,
}
pub const SCREEN_OFFSET: ScreenOffset = ScreenOffset {
    x: ENEMY_SPRITE_WIDTH * -1.,
    y: ENEMY_SPRITE_HEIGHT + (ENEMY_SPRITE_HEIGHT * 0.5),
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    GameOver,
    Active,
    StartMenu,
    Pause,
    Init,
}

#[derive(Resource)]
pub struct GameResources {
    energy: f32,
    score: u32,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_state::<GameState>()
            .add_systems(Update, detect_intersection_player)
            .add_systems(Update, toggle_pause)
            .add_systems(Update, toggle_start)
            .add_systems(Update, burn_energy)
            .add_systems(Update, update_debug_text);
    }
}

fn init(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::StartMenu);
    commands.insert_resource(GameResources {
        energy: 1.,
        score: 0,
    });
}

pub fn burn_energy(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_resources: ResMut<GameResources>,
    controller_state: Res<PlayerControllerState>,
    time: Res<Time>,
) {
    if *game_state.get() == GameState::Active {
        if game_resources.energy <= 0. {
            next_game_state.set(GameState::GameOver);
        } else if controller_state.is_moving() {
            game_resources.energy -= time.delta_seconds() * MOVING_ENERGY_BURNING_RATE;
        } else if controller_state.is_boosting() {
            game_resources.energy -= time.delta_seconds() * SPRINTING_ENERGY_BURNING_RATE;
        } else {
            game_resources.energy -= time.delta_seconds() * IDLE_ENERGY_BURNING_RATE;
        }
    }
}

pub fn toggle_pause(
    mut keys: ResMut<Input<KeyCode>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let start_button_pressed = gamepad_events
        .read()
        .find(|e| e.button_type == GamepadButtonType::Start && e.value > 0.)
        .is_some();
    if !!keys.just_pressed(KeyCode::Escape) || start_button_pressed {
        next_game_state.set(match *game_state.get() {
            GameState::Active => GameState::Pause,
            GameState::Pause => GameState::Active,
            _ => *game_state.get(),
        });
        keys.reset(KeyCode::Escape);
    }
}

pub fn toggle_start(
    mut keys: ResMut<Input<KeyCode>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_resources: ResMut<GameResources>,
) {
    let start_button_pressed = gamepad_events
        .read()
        .find(|e| e.button_type == GamepadButtonType::Start && e.value > 0.)
        .is_some();
    if !!keys.just_pressed(KeyCode::Space) || start_button_pressed {
        if *game_state.get() == GameState::StartMenu {
            next_game_state.set(GameState::Init);
            game_resources.score = 0;
            game_resources.energy = 1.;
        }
        keys.reset(KeyCode::Escape);
    }
}

pub fn update_debug_text(
    mut texts: Query<&mut Text>,
    game_state: Res<State<GameState>>,
    game_resources: Res<GameResources>,
) {
    let state_str = match *game_state.get() {
        GameState::Pause => "Pause",
        GameState::Active => "Active",
        GameState::GameOver => "Game Over",
        GameState::StartMenu => "Start Menu",
        GameState::Init => "Restarting",
    };
    let score: &str = &game_resources.score.to_string();
    let energy: &str = &format!("{:.1}%", 100. * game_resources.energy).to_string();
    for mut text in &mut texts {
        text.sections[0].value = ["State: ", state_str, "\n"].join("").into();
        text.sections[1].value = ["Score: ", score, "\n"].join("").into();
        text.sections[2].value = ["Energy: ", energy].join("").into();
    }
}

pub fn detect_intersection_player(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &Sprite, Entity), With<EnemyEntity>>,
    player_query: Query<
        (&Transform, &Sprite, Entity),
        (With<PlayerEntity>, Without<PlayerAttached>),
    >,
    web_query: Query<(&Transform, &Sprite, Entity), (With<PlayerEntity>, With<PlayerAttached>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_resources: ResMut<GameResources>,
) {
    for player in player_query.iter() {
        let player_pos = player.0.translation;
        let player_size = player.1.custom_size.unwrap_or_default();
        let player_entity = player.2;
        for web in web_query.iter() {
            let web_pos = web.0.translation;
            let web_size = web.1.custom_size.unwrap_or_default();
            let web_entity = web.2;
            for enemy in enemy_query.iter() {
                let enemy_pos = enemy.0.translation;
                let enemy_size = enemy.1.custom_size.unwrap_or_default();
                let enemy_entity = enemy.2;
                match collide(player_pos, player_size, enemy_pos, enemy_size) {
                    Some(Collision::Bottom) => {
                        commands.entity(player_entity).despawn();
                        commands.entity(web_entity).despawn();
                        next_game_state.set(GameState::GameOver);
                    }
                    Some(_collision) => {
                        commands.entity(enemy_entity).despawn();
                        game_resources.score += 1;
                        game_resources.energy += match game_resources.energy {
                            e if (0.0..0.3).contains(&e) => 0.3,
                            e if (0.3..0.7).contains(&e) => 0.2,
                            e if (0.7..0.9).contains(&e) => 0.1,
                            e if (0.9..0.99).contains(&e) => 0.01,
                            _ => 0.,
                        }
                    }
                    _ => {}
                };
                match collide(web_pos, web_size, enemy_pos, enemy_size) {
                    Some(_collision) => {
                        commands.entity(web_entity).despawn();
                        commands.entity(player_entity).despawn();
                        next_game_state.set(GameState::GameOver);
                    }
                    _ => {}
                };
            }
        }
    }
}
