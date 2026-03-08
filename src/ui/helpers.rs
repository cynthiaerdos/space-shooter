use bevy::prelude::*;

pub fn spawn_button(parent: &mut bevy::prelude::ChildSpawnerCommands, label: &str, marker: impl bevy::prelude::Component) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Node::DEFAULT
            },
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            marker,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}