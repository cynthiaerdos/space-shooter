use bevy::prelude::*;

use crate::states::GameState;
use super::{PlayButton};

pub fn handle_play_button_interaction(
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}