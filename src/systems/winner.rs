use crate::{
    components::Ball,
    pong::{ ARENA_WIDTH, ARENA_HEIGHT, ScoreBoard, ScoreText },
    audio::{ self, Sounds,}
};
use amethyst::{
    derive::SystemDesc,
    core::Transform,
    ecs::{ System, SystemData, Write, WriteStorage, Join, Read, ReadExpect, },
    ui::UiText,
    assets::AssetStorage,
    audio::{ output::Output, Source }
};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    ); 

    fn run(&mut self, (mut balls, mut transforms, mut ui_text, mut scores, score_text, storage, sounds, audio_output): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.score_right) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.score_left) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                audio::play_score_sound(&*sounds, &storage, audio_output.as_deref());
                println!("Score: | {:^3} | {:^3} |", scores.score_left, scores.score_right);
                ball.velocity.set_x(-ball.velocity.x());
                transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
            }
        }
    }
}

