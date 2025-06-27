//the mod file of the game folder, it load other files and folders in it
mod resources;
mod systems;
mod tower;
mod enemy;
mod gold;

use {resources::*, systems::*};
use enemy::EnemyPlugin;
use tower::TowerPlugin;
use gold::GoldPlugin;

use bevy::prelude::*;

//the GamePlugin here to initialise and use the different component, systems (ect.) in it
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Map>()
            .add_plugins(EnemyPlugin)
            .add_plugins(TowerPlugin)
            .add_plugins(GoldPlugin)

            .add_systems(Startup, (select_map.before(setup), setup));
    }
}