use bevy::prelude::*;

use super::{ScoreText};
use crate::shared::resources::{Score};

pub fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }
    for mut text in &mut query {
        **text = format!("Score: {}", score.value);
    }
}