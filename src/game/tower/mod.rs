//the mod file of the tower folder, it load other files and folders in it
pub mod components;
pub mod resources;
pub mod archer;
pub mod boomer;
pub mod knight;
pub mod events;
mod systems;

use resources::*;
use systems::*;
use events::*;

use bevy::prelude::*;

//the TowerPlugin here to initialise and use the different components, resources, systems and events in it
pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlacementState::default())
            .add_event::<TowerPlacementRequest>()
            .add_systems(Startup, spawn_text_entity)
            .add_systems(Update,(
                on_mouse_click,
                handle_tower_placement,
                tower_attack,
            ))
            .add_systems(Update, on_keyboard_key_pressed);
    }
}