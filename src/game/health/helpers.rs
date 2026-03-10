use super::{HealthBar};

use bevy::prelude::*;

pub fn spawn_health_bar(commands: &mut Commands, target: Entity) {
    let width = 70.0;
    let height = 6.0;
    let offset = Vec2::new(0.0, -40.0);

    commands.spawn((
        HealthBar { target, offset, width, height },
        Sprite {
            color: Color::srgba(0.0, 1.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}