//the systems.rs file is for the systems of the enemy using resources in resources.rs
use super::components::*;
use super::resources::*;

use bevy::prelude::*;

const DISPLAY_TILE_SIZE: f32 = 32.0;

//the map_to_world function transform map coordonates into world coordonate
fn map_to_world(
    x: usize,
    y: usize
) -> Vec2 {
    let world_x = -(40.0 * DISPLAY_TILE_SIZE) / 2.0 + DISPLAY_TILE_SIZE / 2.0 + x as f32 * DISPLAY_TILE_SIZE;
    let world_y = (24.0 * DISPLAY_TILE_SIZE) / 2.0 - DISPLAY_TILE_SIZE / 2.0 - y as f32 * DISPLAY_TILE_SIZE;
    Vec2::new(world_x, world_y)
}

//the spawn_enemy system spawn a simple enemy with a path to follow, his health depending the EnemySpawnTimer's multiplier
pub fn spawn_enemy(
    spawn_multiplier: &mut ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let raw_path = vec![
        (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11),
        (9, 12), (9, 13), (9, 14), (9, 15), (9, 16), (9, 17),
        (10, 17), (11, 17), (12, 17), (13, 17), (14, 17), (15, 17), (16, 17), (17, 17), (18, 17), (19, 17), (20, 17),
        (20, 16), (20, 15), (20, 14), (20, 13), (20, 12), (20, 11),
        (21, 11), (22, 11), (23, 11), (24, 11), (25, 11), (26, 11), (27, 11), (28, 11), (29, 11), (30, 11), (31, 11), (32, 11), (33, 11), (34, 11), (35, 11), (36, 11), (37, 11), (38, 11), (39, 11),
    ];

    let waypoints: Vec<Vec2> = raw_path.into_iter().map(|(x, y)| map_to_world(x, y)).collect();

    let current_health = (10.0 * spawn_multiplier.multiplier) as usize;

    let enemy_entity = commands.spawn((
        Sprite {
            image: asset_server.load("sprites/tank_red.png"),
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform {
            translation: waypoints[0].extend(2.0),
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..default()
        },
        Enemy {
            health: current_health,
        },
        Path {
            waypoints,
            current: 0,
        },
        Velocity {
            direction: Vec2::ZERO,
            speed: 100.0,
        },
    ))
    .id();

    commands.entity(enemy_entity).with_children(|parent| {
        parent.spawn((
            Text2d::new((10 as f32 *spawn_multiplier.multiplier).to_string()),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE.into()),
            HealthBar,
        ));
    });
}

//the spawn_big_enemy system spawn a big enemy with a path to follow, his health depending the EnemySpawnTimer's multiplier
pub fn spawn_big_enemy(
    spawn_multiplier: &mut ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let raw_path = vec![
        (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11),
        (9, 12), (9, 13), (9, 14), (9, 15), (9, 16), (9, 17),
        (10, 17), (11, 17), (12, 17), (13, 17), (14, 17), (15, 17), (16, 17), (17, 17), (18, 17), (19, 17), (20, 17),
        (20, 16), (20, 15), (20, 14), (20, 13), (20, 12), (20, 11),
        (21, 11), (22, 11), (23, 11), (24, 11), (25, 11), (26, 11), (27, 11), (28, 11), (29, 11), (30, 11), (31, 11), (32, 11), (33, 11), (34, 11), (35, 11), (36, 11), (37, 11), (38, 11), (39, 11),
    ];

    let waypoints: Vec<Vec2> = raw_path.into_iter().map(|(x, y)| map_to_world(x, y)).collect();

    let current_health = (50.0 * spawn_multiplier.multiplier) as usize;

    let enemy_entity = commands.spawn((
        Sprite {
            image: asset_server.load("sprites/tank_dark.png"),
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform {
            translation: waypoints[0].extend(2.0),
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..default()
        },
        Enemy {
            health: current_health,
        },
        Path {
            waypoints,
            current: 0,
        },
        Velocity {
            direction: Vec2::ZERO,
            speed: 75.0,
        },
    ))
    .id();

    commands.entity(enemy_entity).with_children(|parent| {
        parent.spawn((
            Text2d::new((50 as f32 *spawn_multiplier.multiplier).to_string()),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE.into()),
            HealthBar,
        ));
    });
}

//the move_enemies system make the enemies moove with a certain speed
pub fn move_enemies(
    mut query: Query<(&Velocity, &mut Transform), With<Enemy>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in &mut query {
        let movement = velocity.direction * velocity.speed * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

//the follow_path system make the enemies follow their path
pub fn follow_path(
    mut query: Query<(&mut Transform, &mut Path, &mut Velocity), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, mut path, mut velocity) in &mut query {
        if path.current >= path.waypoints.len() {
            velocity.direction = Vec2::ZERO;
            continue;
        }

        let target = path.waypoints[path.current];
        let current_pos = transform.translation.truncate();

        let direction = (target - current_pos).normalize_or_zero();
        velocity.direction = direction;

        let distance = velocity.speed * time.delta_secs();
        let distance_to_target = current_pos.distance(target);

        if distance >= distance_to_target {
            transform.translation = target.extend(2.0);
            path.current += 1;
        } else {
            transform.translation += (direction * distance).extend(2.0);
        }
    }
}

//the update_health_texts system is for update the text which is the health_bar
pub fn update_health_texts(
    mut text_query: Query<&mut Text2d, With<HealthBar>>,
    enemy_query: Query<(&Enemy, &Children)>,
) {
    for (enemy, children) in &enemy_query {
        for &child in children {
            if let Ok(mut text) = text_query.get_mut(child) {
                let new_health_text = enemy.health.to_string();
                if text.0 != new_health_text {
                    text.0 = new_health_text;
                }
            }
        }
    }
}

//the tick_enemy_spawn_time system here to tick the timer every frame
pub fn tick_enemy_spawn_time(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

//the spawn_enemy_over_time system here to make simple or big enemies spawn depending of the counter
pub fn spawn_enemy_over_time(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    if enemy_spawn_timer.timer.finished(){
        if enemy_spawn_timer.counter < 15 {
            enemy_spawn_timer.counter += 1;
            spawn_enemy(&mut enemy_spawn_timer, asset_server, commands);
        } else {
            enemy_spawn_timer.counter = 0;
            spawn_big_enemy(&mut enemy_spawn_timer, asset_server, commands);
            enemy_spawn_timer.multiplier += 0.5;
        }
    }
}

//the end_path system check if the path of enemies is end and despawn the entity
pub fn end_path(
    enemy_query: Query<(Entity, &Path), (With<Enemy>, With<Path>)>,
    mut commands: Commands,
) {
    for (enemy_entity, path) in &enemy_query {
        if path.current == path.waypoints.len() {
            commands.entity(enemy_entity).despawn();
        }
    }
}

//the cleanup_marked_entities system is here to cleanup the enemies when they die to not have problems with double despawn
pub fn cleanup_marked_entities(
    query: Query<Entity, With<MarkedForDespawn>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
