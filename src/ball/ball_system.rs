use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::ball::component::Ball;
use crate::state::Pause;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, Pause>,
    );

    fn run(&mut self, (balls, mut locals, time, pause): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        if pause.paused {
            return;
        }
        for (ball, local) in (&balls, &mut locals).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}
