use bevy::prelude::*;

use super::{Projectile};
use crate::shared::components::{ DespawnOffscreen, Velocity};
use crate::shared::constants::{BOUNDS_X, BOUNDS_Y};

pub fn move_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

pub fn despawn_offscreen_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Projectile>, With<DespawnOffscreen>)>,
) {
    for (entity, transform) in &query {
        let pos = transform.translation;

        let out_of_y_bounds = pos.y > BOUNDS_Y + 50.0 || pos.y < -BOUNDS_Y - 50.0;
        let out_of_x_bounds = pos.x > BOUNDS_X + 50.0 || pos.x < -BOUNDS_X - 50.0;
        
        if out_of_y_bounds || out_of_x_bounds {
            commands.entity(entity).despawn();
        }
    }
}
