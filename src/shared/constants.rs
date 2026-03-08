use bevy::prelude::*;

// Window / world bounds
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const BOUNDS_X: f32 = (WINDOW_WIDTH as f32) / 2.0;
pub const BOUNDS_Y: f32 = (WINDOW_HEIGHT as f32) / 2.0;

// Player
pub const PLAYER_DIMENSION: Vec2 = Vec2::new(60.0, 46.0);
pub const PLAYER_LIVES: u32 = 3;
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_Y_COORDINATE: f32 = -BOUNDS_Y + 60.0;
pub const PLAYER_PROJECTILES_PER_SECOND: f32 = 4.0;
pub const PLAYER_PROJECTILE_SPEED_PER_SECOND: f32 = 1000.0;

// Enemies
pub const ENEMY_DIMENSION: Vec2 = Vec2::new(55.0, 48.0);
pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_SPAWNS_PER_SECOND: f32 = 0.8;
pub const ENEMY_MAX_COUNT: usize = 12;
pub const ENEMY_PROJECTILES_PER_SECOND: f32 = 0.8;
pub const ENEMY_PROJECTILE_SPEED_PER_SECOND: f32 = 300.0;

// Scores
pub const SCORE_PER_ENEMY: u32 = 100;

// Collision radius
pub const PROJECTILE_RADIUS: f32 = 7.0;