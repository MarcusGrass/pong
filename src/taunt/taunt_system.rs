use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{ReadStorage, System, SystemData, WriteStorage, WriteExpect};

use crate::ball::component::Ball;
use crate::taunt::Taunt;
use amethyst::core::ecs::{Join, Read};
use amethyst::renderer::SpriteRender;
use crate::persistence::Settings;

#[derive(SystemDesc)]
pub struct TauntSystem;

impl<'s> System<'s> for TauntSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        WriteExpect<'s, Taunt>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Settings>,
    );

    fn run(&mut self, (balls, trans, taunt, mut sprites, settings): Self::SystemData) {
        for (_ball, transform) in (&balls, &trans).join() {
            let md = settings.window_settings.arena_width() / 3.0;
            let pos = transform.translation().x / md;
            let sprite = sprites.get_mut(taunt.face.unwrap()).unwrap();
            let sprite_ind = if pos < 1.0 {
                2
            } else if pos < 2.0 {
                4
            } else {
                3
            };
            if sprite.sprite_number != sprite_ind {
                sprite.sprite_number = sprite_ind;
            }

        }

    }
}
