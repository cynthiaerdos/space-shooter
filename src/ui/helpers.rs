use bevy::prelude::*;

use crate::shared::resources::{FontAssets};

pub fn spawn_button(parent: &mut bevy::prelude::ChildSpawnerCommands, label: &str, marker: impl bevy::prelude::Component, fonts: &Res<FontAssets>) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
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
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}