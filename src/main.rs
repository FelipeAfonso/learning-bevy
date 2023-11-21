use bevy::prelude::*;

mod controllers;
mod entities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(entities::EntitiesPlugin)
        .add_plugin(controllers::ControllersPlugin)
        .run();
}
