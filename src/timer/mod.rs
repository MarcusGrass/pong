pub mod timer_system;

use amethyst::core::ecs::Entity;

#[derive(Default)]
pub struct TimerText {
    pub game_time: f32,
    pub timer: Option<Entity>,
}
