
use bevy::prelude::*;

use crate::score::resources::*;
use crate::events::GameOver;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>
) {
    game_over_event_reader.read().for_each(|event| {
        high_scores.scores.push(("Player".to_string(), event.score));
    })
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}