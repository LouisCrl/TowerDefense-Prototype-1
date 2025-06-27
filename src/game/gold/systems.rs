//the systems.rs file is for the systems of the gold using resources in resources.rs
use super::components::*;
use super::resources::*;

use bevy::prelude::*;

//the spawn_text_gold_entity system is here to spawn the text indicating the amount of gold you have
pub fn spawn_text_gold_entity (
    mut commands: Commands,
) {
    commands.spawn((
        Text2d::new("10".to_string()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE.into()),
        Transform {
            translation: Vec3::new(610.0, 330.0, 3.0),
            ..default()
        },
        GoldTextEntity,
    ));
}

//the update_text_gold_entity is to update the printed amount of gold
pub fn update_text_gold_entity (
    mut text_query: Query<&mut Text2d, With<GoldTextEntity>>,
    golds: Res<Gold>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        text.0 = golds.value.to_string();
    }
}