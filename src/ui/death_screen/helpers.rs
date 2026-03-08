use bevy::prelude::*;

use super::{RestartButton, MenuButton, DeathScreenRoot};
use crate::ui::helpers;
use crate::shared::resources::{Score};

pub fn spawn_death_screen(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                ..default()
            },
            DeathScreenRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 52.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));

            parent.spawn((
                Text::new(format!("Final Score: {}", score.value)),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            helpers::spawn_button(parent, "RESTART", RestartButton);
            helpers::spawn_button(parent, "MENU", MenuButton);
        });
}