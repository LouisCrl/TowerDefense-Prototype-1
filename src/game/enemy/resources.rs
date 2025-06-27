//the resources.rs file is for the resources of the enemy used in systems.rs
use bevy::prelude::*;

pub const ENEMY_SPAWN_TIMER: f32 = 1.0;

//the EnemySpawnTimer resource is a timer with a counter and a multiplier which impact enemies
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
    pub counter: usize,
    pub multiplier: f32,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
            counter: 0,
            multiplier: 1.0,
        }
    }
}