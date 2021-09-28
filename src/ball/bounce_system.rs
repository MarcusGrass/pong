use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::ReadExpect;

use crate::audio::audio::{play_bounce_sound, Sounds};
use crate::ball::component::Ball;
use crate::paddle::component::{Paddle, Side};
use crate::persistence::{Settings};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        Read<'s, Settings>,
    );

    fn run(
        &mut self,
        (mut balls, paddles, transforms, storage, sounds, audio_output, settings): Self::SystemData,
    ) {
        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        let window_settings = settings.window_settings;
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= window_settings.arena_height() - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
            }

            // Don't do unnecessary calc if far away
            if ball_x > window_settings.paddle_width() + window_settings.ball_radius() && ball_x < window_settings.arena_width() - window_settings.paddle_width() - window_settings.ball_radius() {
                return;
            }

            // Bounce at the paddles.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {

                        let mut speed_mod = 0.0;
                        if ball.velocity[0].abs() + window_settings.ball_velocity_x() * 0.1 < window_settings.max_velocity() - window_settings.ball_velocity_x() * 0.1 {
                            speed_mod = 0.035;
                        }
                        if ball.velocity[0].is_sign_positive() {
                            ball.velocity[0] = -ball.velocity[0] - window_settings.ball_velocity_x() * speed_mod;
                        } else {
                            ball.velocity[0] = -ball.velocity[0] + window_settings.ball_velocity_x() * speed_mod;
                        }
                        if ball.velocity[1].is_sign_positive() {
                            ball.velocity[1] = ball.velocity[1] + window_settings.ball_velocity_y() * speed_mod;
                        } else {
                            ball.velocity[1] = ball.velocity[1] - window_settings.ball_velocity_y() * speed_mod;
                        }

                        ball.calculated_impact_y = None;
                        play_bounce_sound(&settings.audio_settings, &*sounds, &storage, audio_output.as_deref());
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
