use bevy::prelude::*;

use crate::ui::helpers;
use crate::shared::resources::FontAssets;

use super::{MainMenuRoot, PlayButton};

pub fn spawn_main_menu(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(30.0),
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Space Shooter"),
                TextFont {
                    font: fonts.mono.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            helpers::spawn_button(parent, "PLAY", PlayButton, &fonts);
        });
}
