use bevy::prelude::*;

mod controllers;
mod entities;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(entities::EntitiesPlugin)
        .add_plugins(controllers::ControllersPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
