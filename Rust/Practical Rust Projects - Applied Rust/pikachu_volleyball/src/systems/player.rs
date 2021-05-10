use amethyst::{
    core::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    core::timing::Time,
};

use crate::pikachuvolleyball::{Player, Side, ARENA_WIDTH, PLAYER_WIDTH};

const PLAYER_SPEED: f32 = 128.0;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );
    fn run(
        &mut self,
        (mut transforms, players, input, time): Self::SystemData
    ) {
        for(player, transform) in (&players, &mut transforms).join() {
            let movement = match player.side {
                Side::Left => input.axis_value("left_player"),
                Side::Right => input.axis_value("right_player")
            };

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = (
                        PLAYER_SPEED *
                        time.delta_seconds() *
                        mv_amount
                    ) as f32;
                    
                    let player_x = transform.translation().x;
                    let player_left_limit = match player.side {
                        Side::Left => 0.0,
                        Side::Right => ARENA_WIDTH * 0.5
                    };
                    
                    transform.set_translation_x(
                        (player_x + scaled_amount)
                            .max(player_left_limit + PLAYER_WIDTH * 0.5)
                            .min(player_left_limit + ARENA_WIDTH * 0.5 - PLAYER_WIDTH * 0.5)
                    );

                    // curr = 110, min_boundary: 0, max_boundary: 100.
                    // 110.max(0) = 110
                    // (110.max(0) = 110).min(100) = 100
                }
            }
        }
    }
}