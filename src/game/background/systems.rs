use bevy::prelude::*;

use super::{ScrollingBackground};
use crate::shared::constants::{SMALL_STAR_BACKGROUND_SPEED, WINDOW_HEIGHT};

pub fn scroll_background(
    time: Res<Time>,
    mut small_star_query: Query<&mut Transform, With<ScrollingBackground>>
) {
    for mut transform in &mut small_star_query {
        transform.translation.y -= SMALL_STAR_BACKGROUND_SPEED * time.delta_secs();

        if transform.translation.y <= -(WINDOW_HEIGHT as f32) {
            transform.translation.y = WINDOW_HEIGHT as f32;
        }
    }
}