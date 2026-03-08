mod projectile;
mod collision;
mod shared;
mod enemy;
mod player;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use shared::resources::{SpriteAssets};
use shared::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Shooter"),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                // Use nearest-neighbour sampling so pixel-art sprites stay crisp.
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(PreStartup, load_assets)
        .add_plugins((
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            enemy::EnemyPlugin,
            collision::CollisionPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SpriteAssets {
        player: asset_server.load("sprites/player.png"),
        enemy: asset_server.load("sprites/enemy.png"),
        player_projectile: asset_server.load("sprites/player_projectile.png"),
        enemy_projectile: asset_server.load("sprites/enemy_projectile.png"),
    });
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}