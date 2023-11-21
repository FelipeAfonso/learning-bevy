use bevy::{
    input::{
        gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent},
        keyboard::KeyboardInput,
    },
    prelude::*,
};

#[derive(Resource)]
pub struct PlayerControllerState {
    x: f32,
    y: f32,
    boost: bool,
}
impl PlayerControllerState {
    pub fn release_move_player(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.y = 0.0,
            KeyCode::Down => self.y = 0.0,
            KeyCode::Left => self.x = 0.0,
            KeyCode::Right => self.x = 0.0,
            KeyCode::LShift => self.boost = false,
            _ => (),
        }
    }
    pub fn move_player(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.y = -1.0,
            KeyCode::Down => self.y = 1.0,
            KeyCode::Left => self.x = -1.0,
            KeyCode::Right => self.x = 1.0,
            KeyCode::LShift => self.boost = true,
            _ => (),
        }
    }
    pub fn move_player_joystick(&mut self, val: f32, axis: GamepadAxisType) {
        match axis {
            GamepadAxisType::LeftStickX => self.x = val,
            GamepadAxisType::LeftStickY => self.y = val * -1.0,
            _ => (),
        }
    }
    pub fn move_player_joystick_buttons(&mut self, val: f32, button: GamepadButtonType) {
        match button {
            GamepadButtonType::RightTrigger2 => {
                if val > 0.5 {
                    self.boost = true
                } else {
                    self.boost = false
                }
            }
            _ => (),
        }
    }
    pub fn is_boosting(&self) -> bool {
        self.boost
    }
    pub fn get_state(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

pub struct ControllersPlugin;
impl Plugin for ControllersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(player_controller);
    }
}

pub fn player_controller(
    mut key_evr: EventReader<KeyboardInput>,
    mut joy_evr: EventReader<GamepadAxisChangedEvent>,
    mut joy_b_evr: EventReader<GamepadButtonChangedEvent>,
    mut state: ResMut<PlayerControllerState>,
) {
    use bevy::input::ButtonState;
    for ev in joy_evr.iter() {
        state.move_player_joystick(ev.value, ev.axis_type)
    }
    for ev in joy_b_evr.iter() {
        state.move_player_joystick_buttons(ev.value, ev.button_type)
    }

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => state.move_player(ev.key_code.unwrap_or(KeyCode::Space)),
            ButtonState::Released => {
                state.release_move_player(ev.key_code.unwrap_or(KeyCode::Space))
            }
        }
    }
}
pub fn setup(mut commands: Commands) {
    commands.insert_resource(PlayerControllerState {
        x: 0.0,
        y: 0.0,
        boost: false,
    });
}
