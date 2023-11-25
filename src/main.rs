use bevy::prelude::*;

mod controllers;
mod entities;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(entities::EntitiesPlugin)
        .add_plugin(controllers::ControllersPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}
