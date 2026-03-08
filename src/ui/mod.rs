pub mod main_menu;
pub mod hud;
pub mod death_screen;
pub mod helpers;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_menu::MainMenuPlugin,
            hud::HudPlugin,
            death_screen::DeathScreenPlugin,
        ));
    }
}