use bevy::prelude::*;

use super::{ScrollingBackground};
use crate::shared::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
use crate::shared::resources::{SpriteAssets};

pub fn setup_background(mut commands: Commands, sprites: Res<SpriteAssets>) {
    for i in 0..2 {
        commands.spawn((
            ScrollingBackground,
            Sprite {
                image: sprites.background_with_small_stars.clone(),
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                image_mode: SpriteImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 1.0,
                },
                custom_size: Some(Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                ..default()
            },
            Transform::from_xyz(0.0,i as f32 * WINDOW_HEIGHT as f32, -10.0),
        ));
    }
}
