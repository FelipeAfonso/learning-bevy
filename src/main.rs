use bevy::prelude::*;

mod entities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(entities::EntitiesPlugin)
        .run();
}
