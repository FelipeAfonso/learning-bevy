use bevy::prelude::*;

mod controllers;
mod entities;
mod game;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(entities::EntitiesPlugin)
        .add_plugins(controllers::ControllersPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
