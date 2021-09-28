use amethyst::core::Time;
use amethyst::core::ecs::{WriteStorage, System, Read, WriteExpect, World};
use amethyst::ui::UiText;
use crate::timer::TimerText;
use amethyst::prelude::SystemDesc;
use amethyst::core::ecs::shred::SystemData;
use crate::state::Pause;

pub struct TimerSystem;

impl<'a, 'b> SystemDesc<'a, 'b, TimerSystem> for TimerSystem {
    fn build(self, world: &mut World) -> TimerSystem {
        <TimerSystem as System>::SystemData::setup(world);
        TimerSystem
    }
}

impl<'s> System<'s> for TimerSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteExpect<'s, TimerText>,
        WriteStorage<'s, UiText>,
        Read<'s, Pause>,
    );

    fn run(&mut self, (time, mut timer_text, mut ui_text, pause): Self::SystemData) {
        if pause.paused {
            return;
        }
        if let Some(timer) = timer_text.timer {
            let ui = ui_text.get_mut(timer).unwrap();
            let as_float = ui.text.parse::<f32>().unwrap();
            timer_text.game_time += time.delta_seconds();
            if timer_text.game_time - as_float > 0.1 {
                ui.text = format!("{:.1}", timer_text.game_time);
            }
        }
    }
}
