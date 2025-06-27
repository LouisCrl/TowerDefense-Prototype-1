//the components.rs file is for the components concerning knights
use bevy::prelude::*;

//the Knight component here to get the stats of the knight tower
#[derive(Component, Clone, Copy, PartialEq)]
pub struct Knight {
    pub attack_damage: usize,
    pub attack_speed: f32,
    pub attack_range: f32,
    pub damage_radius: f32,
    pub around_aoe: bool,
}

impl Default for Knight {
    fn default() -> Self {
        Knight {
            attack_damage: 5,
            attack_range: 2.0,
            attack_speed: 1.0,
            damage_radius: 0.0,
            around_aoe: true,
        }
    }
}