use bevy::prelude::*;

use super::{HudRoot, ScoreText};

pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(14.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));
        });
}