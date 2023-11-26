use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::entities::{EnemyEntity, PlayerAttached, PlayerEntity};

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
        app.add_startup_system(init)
            .add_state::<GameState>()
            .add_system(detect_intersection_player)
            .add_system(toggle_pause)
            .add_system(toggle_start)
            .add_system(burn_energy)
            .add_system(update_debug_text);
    }
}

fn init(mut commands: Commands, mut state: ResMut<State<GameState>>) {
    state.0 = GameState::StartMenu;
    commands.insert_resource(GameResources {
        energy: 1.,
        score: 0,
    });
}

pub fn burn_energy(
    mut game_state: ResMut<State<GameState>>,
    mut game_resources: ResMut<GameResources>,
    time: Res<Time>,
) {
    if game_state.0 == GameState::Active {
        if game_resources.energy <= 0. {
            game_state.0 = GameState::GameOver;
        } else {
            game_resources.energy -= time.delta_seconds() * 0.1;
        }
    }
}

pub fn toggle_pause(
    mut keys: ResMut<Input<KeyCode>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut state: ResMut<State<GameState>>,
) {
    let start_button_pressed = gamepad_events
        .iter()
        .find(|e| e.button_type == GamepadButtonType::Start && e.value > 0.)
        .is_some();
    if !!keys.just_pressed(KeyCode::Escape) || start_button_pressed {
        state.0 = match state.0 {
            GameState::Active => GameState::Pause,
            GameState::Pause => GameState::Active,
            _ => state.0,
        };
        keys.reset(KeyCode::Escape);
    }
}

pub fn toggle_start(
    mut keys: ResMut<Input<KeyCode>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut state: ResMut<State<GameState>>,
    mut game_resources: ResMut<GameResources>,
) {
    let start_button_pressed = gamepad_events
        .iter()
        .find(|e| e.button_type == GamepadButtonType::Start && e.value > 0.)
        .is_some();
    if !!keys.just_pressed(KeyCode::Escape) || start_button_pressed {
        if state.0 == GameState::StartMenu {
            state.0 = GameState::Init;
            game_resources.score = 0;
            game_resources.energy = 1.;
        }
        keys.reset(KeyCode::Escape);
    }
}

pub fn update_debug_text(
    mut texts: Query<&mut Text>,
    state: Res<State<GameState>>,
    game_resources: Res<GameResources>,
) {
    let state_str = match state.0 {
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
    mut game_state: ResMut<State<GameState>>,
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
                        game_state.0 = GameState::GameOver;
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
                        game_state.0 = GameState::GameOver;
                    }
                    _ => {}
                };
            }
        }
    }
}
