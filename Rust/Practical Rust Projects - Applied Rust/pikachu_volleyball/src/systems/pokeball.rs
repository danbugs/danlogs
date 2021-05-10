use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, World, WriteStorage},
};

use crate::pikachuvolleyball::Pokeball;

#[derive(SystemDesc)]
pub struct PokeballSystem;

pub const GRAVITY_ACCELERATION: f32 = -108.0;

impl<'s> System<'s> for PokeballSystem {
    type SystemData = (
        WriteStorage<'s, Pokeball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut pokeballs, mut locals, time): Self::SystemData) {
        for (pokeball, local) in (&mut pokeballs, &mut locals).join() {
            local.prepend_translation_x(pokeball.velocity.0 * time.delta_seconds());
            local.prepend_translation_y((pokeball.velocity.1 + time.delta_seconds() * GRAVITY_ACCELERATION) * time.delta_seconds());

            pokeball.velocity.1 = pokeball.velocity.1 + time.delta_seconds() * GRAVITY_ACCELERATION;
        }
    }
}