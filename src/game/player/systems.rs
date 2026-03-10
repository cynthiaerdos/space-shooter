use bevy::prelude::*;

use super::{Player, ProjectileCooldown};
use crate::game::health::Health;
use crate::game::projectile;
use crate::shared::constants::*;
use crate::shared::resources::{SpriteAssets};
use crate::states::GameState;

pub fn check_player_health(
    mut game_state: ResMut<NextState<GameState>>,
      mut query: Query<&mut Health, With<Player>>,
) {
    let Ok(health) = query.single_mut() else {
        return;
    };

    if health.current < 0.0 {
        game_state.set(GameState::Dead);
    }
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let direction = get_movement_direction(keyboard);

    transform.translation.x += direction * PLAYER_SPEED * time.delta_secs();

    limit_coordinates_to_bounds(transform);
}


pub fn player_projectile_shooting(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
    mut commands: Commands,
    mut query: Query<(&Transform, &mut ProjectileCooldown), With<Player>>,
) {
    let Ok((transform, mut cooldown)) = query.single_mut() else {
        return;
    };

    cooldown.timer.tick(time.delta());

    let is_space_pressed = keyboard.pressed(KeyCode::Space);

    if is_space_pressed && cooldown.timer.is_finished() {
        cooldown.timer.reset();

        projectile::helpers::spawn_player_bullet(
            &mut commands,
            &sprites,
            transform.translation.truncate(),
        );
    }
}

fn get_movement_direction(keyboard: Res<ButtonInput<KeyCode>>) -> f32 {
    let arrow_left_pressed = keyboard.pressed(KeyCode::ArrowLeft);
    let key_a_pressed = keyboard.pressed(KeyCode::KeyA);

    if arrow_left_pressed || key_a_pressed {
        return -1.0;
    }

    let arrow_right_pressed = keyboard.pressed(KeyCode::ArrowRight);
    let key_d_pressed = keyboard.pressed(KeyCode::KeyD);

    if arrow_right_pressed || key_d_pressed {
        return 1.0;
    }

    return 0.0;
}


fn limit_coordinates_to_bounds(mut transform: Mut<'_, Transform>) {
    transform.translation.x = transform
        .translation
        .x
        .clamp(-BOUNDS_X + 30.0, BOUNDS_X - 30.0);
}