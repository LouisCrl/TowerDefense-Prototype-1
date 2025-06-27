//the components.rs file is for the components concerning enemies
use bevy::prelude::*;

//the Enemy component with an health
#[derive(Component)]
pub struct Enemy {
    pub health: usize,
}

//the Velocity component to have speed and direction
#[derive(Component)]
pub struct Velocity {
    pub direction: Vec2,
    pub speed: f32,
}

//the Path component to get the Path
#[derive(Component)]
pub struct Path {
    pub waypoints: Vec<Vec2>,
    pub current: usize,
}

//the HealthBar component to have the health printed
#[derive(Component)]
pub struct HealthBar;

//the MarkedForDespawn component to despawn enemies which are marked
#[derive(Component)]
pub struct MarkedForDespawn;
