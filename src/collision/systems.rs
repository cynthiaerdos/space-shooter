use bevy::prelude::*;
use bevy::math::bounding::{BoundingCircle, IntersectsVolume};

use crate::projectile::{EnemyProjectile, PlayerProjectile};
use crate::shared::components::RadiusCollider;
use crate::enemy::Enemy;
use crate::player::Player;

pub fn player_projectile_hits_enemy(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &RadiusCollider), With<PlayerProjectile>>,
    enemy_query: Query<(Entity, &Transform, &RadiusCollider), With<Enemy>>,
) {
    for (projectile_entity, projectile_position, projectile_collider) in &projectile_query {
        for (enemy_entity, enemy_position, enemy_collider) in &enemy_query {
            if check_circle_collision(
                projectile_position.translation.truncate(),
                projectile_collider.radius,
                enemy_position.translation.truncate(),
                enemy_collider.radius,
            ) {
                commands.entity(projectile_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn enemy_projectile_hits_player(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &RadiusCollider), With<EnemyProjectile>>,
    player_query: Query<(&Transform, &RadiusCollider), With<Player>>,
) {
    let Ok((player_position, player_collider)) = player_query.single() else {
        return;
    };

    for (projectile_entity, projectile_position, projectile_collider) in &projectile_query {
        if check_circle_collision(
            projectile_position.translation.truncate(),
            projectile_collider.radius,
            player_position.translation.truncate(),
            player_collider.radius,
        ) {
            commands.entity(projectile_entity).despawn();
            warn!("Player hit!");
        }
    }
}

fn check_circle_collision(a_position: Vec2, a_radius: f32, b_position: Vec2, b_radius: f32) -> bool {
    let a = BoundingCircle::new(a_position, a_radius);
    let b = BoundingCircle::new(b_position, b_radius);
    a.intersects(&b)
}
