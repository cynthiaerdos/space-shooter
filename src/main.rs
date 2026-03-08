mod game;
mod shared;
mod states;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use shared::resources::{SpriteAssets, Score, Lives};
use shared::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
use states::GameState;

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
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<Score>()
        .init_resource::<Lives>()
        .add_systems(PreStartup, load_assets)
        .add_plugins((
            game::GamePlugin,
            ui::UiPlugin,
        ))
        .init_state::<GameState>()
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