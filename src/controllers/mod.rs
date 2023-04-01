use bevy::prelude::*;

pub struct ControllersPlugin;
impl Plugin for ControllersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_controller);
    }
}

pub fn camera_controller(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut Camera3dBundle>,
) {
    use bevy::input::ButtonState;
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
                query
            }
        }
    }
}
