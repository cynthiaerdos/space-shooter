use bevy::prelude::*;

use crate::shared::resources::{FontAssets};

pub fn spawn_button(parent: &mut bevy::prelude::ChildSpawnerCommands, label: &str, marker: impl bevy::prelude::Component, fonts: &Res<FontAssets>) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(320.0),
                height: Val::Px(72.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.30, 0.30, 0.70)),
            marker,
        ))
        .insert(BorderColor::all(Color::srgb(0.3, 0.3, 0.6)))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: fonts.mono.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}