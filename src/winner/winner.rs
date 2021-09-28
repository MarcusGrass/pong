use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage},
};
use crate::ball::component::Ball;
use amethyst::core::ecs::{ReadExpect, Read, WriteExpect};
use amethyst::assets::{AssetStorage};
use crate::audio::audio::{Sounds, play_score_sound};
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use crate::timer::TimerText;
use amethyst::ui::UiText;
use crate::state::Pause;
use crate::persistence::{Settings};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        WriteExpect<'s, TimerText>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, Pause>,
        Read<'s, Settings>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        storage,
        sounds,
        audio_output,
        mut timer_text,
        mut ui_text,
        pause,
        settings
    ): Self::SystemData) {
        if pause.paused {
            return;
        }
        let window_settings = settings.window_settings;
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Computer scores.
                true
            } else if ball_x >= window_settings.arena_width() - ball.radius {
                panic!("You broke my game.");
            } else {
                false
            };

            if did_hit {
                if ball.velocity[0].is_sign_positive() {
                    ball.velocity[0] = - window_settings.ball_velocity_x();
                } else {
                    ball.velocity[0] = window_settings.ball_velocity_x();
                }
                ball.velocity[1] = window_settings.ball_velocity_y();
                transform.set_translation_x(window_settings.arena_width() / 2.0); // Reset Position
                transform.set_translation_y(window_settings.arena_height() / 2.0); // Reset Position
                ball.calculated_impact_y = None;
                play_score_sound(&settings.audio_settings, &*sounds, &storage, audio_output.as_deref());
                let text = ui_text.get_mut(timer_text.timer.unwrap()).unwrap();
                timer_text.game_time = 0.0;
                text.text = format!("0.0");
            }
        }
    }
}
