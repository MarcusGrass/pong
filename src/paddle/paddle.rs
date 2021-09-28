use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paddle::component::{Paddle};
// You'll have to mark window_settings.paddle_height() as public in pong.rs
use crate::ball::component::Ball;
use crate::paddle::component::Side::Left;
use crate::state::Pause;
use crate::persistence::Settings;
use crate::persistence::window::WindowSettings;

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
        Read<'s, Pause>,
        Read<'s, Settings>,
    );

    fn run(&mut self, (mut transforms, paddles, input, balls, time, pause, settings): Self::SystemData) {
        if pause.paused {
            return;
        }
        let window_settings = settings.window_settings;
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let paddle_y = transform.translation().y;
            if paddle.side == Left {
                let movement = input.axis_value("left_paddle");
                if let Some(mv_amount) = movement {
                    let scaled_amount = window_settings.paddle_speed() * mv_amount as f32 * time.delta_seconds();
                    transform.set_translation_y(clamp_to_arena(paddle_y + scaled_amount, &window_settings));
                }
                continue;
            }
            for ball in balls.as_slice() {
                if let Some(impact) = ball.calculated_impact_y {
                    let distance = (impact - paddle_y - window_settings.paddle_height() * 0.5).abs();
                    if impact > paddle_y + window_settings.paddle_height() * 0.5 {
                        transform.set_translation_y(paddle_y + calc_to_move(distance, &time, &window_settings));
                    } else if impact < paddle_y - window_settings.paddle_height() * 0.5 {
                        transform.set_translation_y(paddle_y - calc_to_move(distance, &time, &window_settings));
                    }
                } else if ball.velocity[0] < 0.0 {
                    // Move towards middle
                    if paddle_y == window_settings.arena_height() / 2.0 {
                        continue;
                    }
                    let distance = (window_settings.arena_height() / 2.0 - paddle_y).abs();
                    if distance < 2.0 {
                        transform.set_translation_y(window_settings.arena_height() / 2.0);
                        continue;
                    }
                    if paddle_y < window_settings.arena_height() / 2.0 {
                        transform.set_translation_y(paddle_y + calc_to_move(distance, &time, &window_settings));
                    } else if paddle_y > window_settings.arena_height() / 2.0 {
                        transform.set_translation_y(paddle_y - calc_to_move(distance, &time, &window_settings));
                    }
                }
            }
        }
    }
}

fn calc_to_move(distance: f32, time: &Time, window_settings: &WindowSettings) -> f32 {
    let mv = window_settings.paddle_speed() * time.delta_seconds();
    if distance > mv { mv } else if mv > distance { distance } else { 0.0 }
}

fn clamp_to_arena(val: f32, window_settings: &WindowSettings) -> f32 {
    val.min(window_settings.arena_height() - window_settings.paddle_height() * 0.5)
        .max(window_settings.paddle_height() * 0.5)
}
