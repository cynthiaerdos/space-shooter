use super::{Health, HealthBar};

use bevy::prelude::*;

pub fn health_bar_follow(
    target_query: Query<&Transform, Without<HealthBar>>,
    mut bar_query: Query<(&HealthBar, &mut Transform)>,
) {
    for (bar, mut transform) in &mut bar_query {
        let Ok(target_transform) = target_query.get(bar.target) else {
            continue;
        };

        transform.translation.x = target_transform.translation.x + bar.offset.x;
        transform.translation.y = target_transform.translation.y + bar.offset.y;

        if transform.translation.z > 5.0 {
            transform.translation.x -= bar.width / 2.0;
        }
    }
}

pub fn health_bar_update(
    target_query: Query<&Health>,
    mut bar_query: Query<(&HealthBar, &mut Sprite)>,
) {
    for (bar, mut sprite) in &mut bar_query {
        let Ok(health) = target_query.get(bar.target) else {
            continue;
        };

        let fraction = health.current / health.max;

        sprite.custom_size = Some(Vec2::new(bar.width * fraction, bar.height));
        sprite.color = Color::srgba(1.0 - fraction, fraction, 0.0, 1.0);
    }
}