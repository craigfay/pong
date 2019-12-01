use amethyst::{
    prelude::{World,SystemDesc},
    core::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
    assets::AssetStorage,
    audio::{output::Output, Source},

};

use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH};
use crate::audio::{play_score_sound, Sounds};

use std::ops::Deref;

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

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut score_board,
        score_text,
        storage,
        sounds,
        audio_output,
    ): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side.
                // We top the score at 999 to avoid text overlap.
                score_board.score_right = (score_board.score_right + 1).min(999);
                if let Some(ui_text) = ui_text.get_mut(score_text.p2_score) {
                    ui_text.text = score_board.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                // We top the score at 999 to avoid text overlap.
                score_board.score_left = (score_board.score_left + 1).min(999);
                if let Some(ui_text) = ui_text.get_mut(score_text.p1_score) {
                    ui_text.text = score_board.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                // Reset the ball.
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

                // Print the score board.
                println!(
                    "Score: | {:^3} | {:^3} |",
                    score_board.score_left, score_board.score_right
                );
            }
        }
    }
}