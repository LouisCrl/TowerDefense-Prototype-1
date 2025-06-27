//the mod file of the gold folder, it load other files in it
pub mod components;
pub mod resources;
mod systems;

use {resources::*, systems::*};

use bevy::prelude::*;

//the GoldPlugin here to initialise and use the differents components, resources and systems concerning the golds
pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Gold>()
            .add_systems(Startup, spawn_text_gold_entity)
            .add_systems(Update, update_text_gold_entity);
    }
}