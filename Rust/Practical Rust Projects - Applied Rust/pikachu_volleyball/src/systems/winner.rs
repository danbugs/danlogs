use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use crate::pikachuvolleyball::{Pokeball, ScoreBoard, ARENA_HEIGHT, ARENA_WIDTH, ScoreText};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Pokeball>,
        WriteStorage<'s, Transform>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (mut pokeballs, mut locals, mut scores, mut score_text, mut ui_text) : Self::SystemData) {
        for (pokeball, transform) in (&mut pokeballs, &mut locals).join() {
            let pokeball_x = transform.translation().x;
            let pokeball_y = transform.translation().y;

            if pokeball_y <= pokeball.radius {
                if pokeball_x <= ARENA_WIDTH * 0.5 {
                    scores.score_right = (scores.score_right + 1).min(999);
                    if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                        text.text = scores.score_right.to_string();
                    }
                } else {
                    scores.score_left = (scores.score_left + 1).min(999);
                    if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                        text.text = scores.score_left.to_string();
                    }
                }

                transform.set_translation_x(ARENA_WIDTH * 0.5);
                transform.set_translation_y(ARENA_HEIGHT * 0.5);

                pokeball.velocity.0 = -pokeball.velocity.0;
                pokeball.velocity.1 = 0.0;
            }
        }
    }
}
