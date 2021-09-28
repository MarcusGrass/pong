use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::ball::component::Ball;
use crate::persistence::Settings;
use crate::persistence::window::WindowSettings;
use amethyst::shred::Read;

#[derive(SystemDesc)]
pub struct TrajectorySystem;

impl<'s> System<'s> for TrajectorySystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        Read<'s, Settings>,

    );

    fn run(
        &mut self,
        (mut balls, transforms, settings): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            if ball.calculated_impact_y.is_none() && ball.velocity[0] > 0.0 {
                let ball_y = transform.translation().y;
                let impact = calculate_impact_point(ball, (ball_x, ball_y), &settings.window_settings);
                ball.calculated_impact_y.replace(impact);
            }

        }
    }
}

fn calculate_impact_point(ball: &Ball, pos: (f32, f32), window_settings: &WindowSettings) -> f32 {
    let distance_x;
    if ball.velocity[0] < 0.0 {
        distance_x = 2.0 * window_settings.arena_width() - window_settings.paddle_width() - pos.0;
    } else {
        distance_x = window_settings.arena_width() - pos.0;
    }
    let time_to_impact = distance_x / ball.velocity[0].abs();
    let distance_y = ball.velocity[1] * time_to_impact;
    let actual_travel = distance_y.abs() % (2.0 * window_settings.arena_height());
    let distance_y = if distance_y.is_sign_positive() {
        actual_travel
    } else { -actual_travel };
    let end = pos.1 + distance_y;
    if end < 0.0 {
        return (pos.1 + distance_y).abs();
    }
    if end > window_settings.arena_height() {
        return 2.0 * window_settings.arena_height() - end;
    }
    end

}
