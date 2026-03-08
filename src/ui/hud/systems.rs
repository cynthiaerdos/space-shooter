use bevy::prelude::*;

use super::{LivesText, ScoreText};
use crate::shared::resources::{Lives, Score};

pub fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }
    for mut text in &mut query {
        **text = format!("Score: {}", score.value);
    }
}

pub fn update_lives(lives: Res<Lives>, mut query: Query<&mut Text, With<LivesText>>) {
    if !lives.is_changed() {
        return;
    }
    for mut text in &mut query {
        **text = format!("Lives: {}", lives.value);
    }
}