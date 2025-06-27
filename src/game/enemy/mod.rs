//the mod file of the enemy folder, it load other files in it
pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::prelude::*;

//the EnemyPlugin here to initialise and use the different components, resources and systems concerning the enemies
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, spawn_enemy)
            .insert_resource(EnemySpawnTimer::default())
            .add_systems(Update,
                (
                    move_enemies,
                    follow_path,
                    update_health_texts,
                    spawn_enemy_over_time,
                    tick_enemy_spawn_time,
                    end_path
                )
            )
            .add_systems(PostUpdate, cleanup_marked_entities);
    }
}