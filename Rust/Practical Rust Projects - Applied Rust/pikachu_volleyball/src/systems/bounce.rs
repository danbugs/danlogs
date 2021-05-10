use amethyst::{core::SystemDesc, 
    core::transform::Transform, 
    derive::SystemDesc, 
    ecs::prelude::{Join, ReadExpect, Read, ReadStorage, System, SystemData, World, WriteStorage},
    assets::AssetStorage,
    audio::{output::Output, Source},
};

use std::ops::Deref;
use crate::audio::{play_bounce, play_score, Sounds};

use crate::pikachuvolleyball::{Pokeball, Player, Side, ARENA_HEIGHT, ARENA_WIDTH};
use rand::Rng;

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Pokeball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (mut pokeballs, players, transforms, storage, sounds, audio_output
    ): Self::SystemData) {
        for (pokeball, transform) in (&mut pokeballs, &transforms).join() {
            let pokeball_x = transform.translation().x;
            let pokeball_y = transform.translation().y;

            if pokeball_y <= pokeball.radius && pokeball.velocity.1 < 0.0 {
                play_score(
                    &sounds,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref())
                );
                pokeball.velocity.1 = -pokeball.velocity.1;
            } else if pokeball_y >= (ARENA_HEIGHT - pokeball.radius) && pokeball.velocity.1 > 0.0 {
                pokeball.velocity.1 = -pokeball.velocity.1;
            } else if pokeball_x <= pokeball.radius && pokeball.velocity.0 < 0.0 {
                pokeball.velocity.0 = -pokeball.velocity.0;
            } else if pokeball_x >= (ARENA_WIDTH - pokeball.radius) && pokeball.velocity.0 > 0.0 {
                pokeball.velocity.0 = -pokeball.velocity.0;
            }

            for (player, player_transform) in (&players, &transforms).join() {
                let player_x = player_transform.translation().x - (player.width * 0.5);
                let player_y = player_transform.translation().y - (player.height * 0.5);
                let mut rng = rand::thread_rng();
                if point_in_rect(
                    pokeball_x,
                    pokeball_y,
                    player_x - pokeball.radius,
                    player_y - pokeball.radius,
                    player_x + player.width + pokeball.radius,
                    player_y + player.height + pokeball.radius,
                ) {
                    if pokeball.velocity.1 < 0.0 {
                        play_bounce(
                            &sounds,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref())
                        );
                        pokeball.velocity.0 = -rng.gen_range(0.6, 1.4) * pokeball.velocity.0;
                        pokeball.velocity.1 = -pokeball.velocity.1;
                    }
                }
            }

        }
    }
}

fn point_in_rect(
    x: f32,
    y: f32,
    left: f32,
    bottom: f32,
    right: f32, 
    top: f32
) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}