//this file is the main.rs file with the main function in it
mod game;

use game::GamePlugin;

use bevy::prelude::*;

//main function necessary in every Rust programs
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tower Defense".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}