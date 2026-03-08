use bevy::prelude::*;

use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

pub fn button_visual_system(
    mut commands: Commands,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    if query.is_empty() {
        return;
    }

    let window_entity = window.single().unwrap();
    let mut hovered = false;

    for (interaction, mut bg, mut border) in &mut query {
        match interaction {
            Interaction::Pressed => {
                hovered = true;
                *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.9));
                *border = BorderColor::all(Color::srgb(0.6, 0.6, 1.0));
            }
            Interaction::Hovered => {
                hovered = true;
                *bg = BackgroundColor(Color::srgb(0.4, 0.4, 0.8));
                *border = BorderColor::all(Color::srgb(0.5, 0.5, 0.9));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.25, 0.25, 0.45));
                *border = BorderColor::all(Color::srgb(0.35, 0.35, 0.55));
            }
        }
    }

    if hovered {
        commands.entity(window_entity).insert(CursorIcon::System(SystemCursorIcon::Pointer));
    } else {
        commands.entity(window_entity).insert(CursorIcon::System(SystemCursorIcon::Default));
    }
}