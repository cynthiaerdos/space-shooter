mod game;
mod shared;
mod states;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use shared::resources::{SpriteAssets, Score, Lives};
use shared::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
use states::GameState;

use crate::shared::resources::FontAssets;

fn main() {
    let mut app = App::new();

    app.add_plugins(
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
        );

    let asset_server = app.world().resource::<AssetServer>().clone();

    app.insert_resource(FontAssets {
        mono: asset_server.load("fonts/FiraMono-Medium.ttf"),
    });

    app.insert_resource(SpriteAssets {
        player: asset_server.load("sprites/player.png"),
        enemy: asset_server.load("sprites/enemy.png"),
        player_projectile: asset_server.load("sprites/player_projectile.png"),
        enemy_projectile: asset_server.load("sprites/enemy_projectile.png"),
        background_with_small_stars: asset_server.load("sprites/background_with_small_stars.png"),
    });
        
    app.insert_resource(ClearColor(Color::srgb(0.125, 0.125, 0.302)))
        .init_resource::<Score>()
        .init_resource::<Lives>()
        .add_plugins((
            game::GamePlugin,
            ui::UiPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}