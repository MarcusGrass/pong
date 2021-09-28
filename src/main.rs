mod state;
mod timer;
mod audio;
mod winner;
mod ball;
mod paddle;
mod taunt;
mod persistence;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Result,
};
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::audio::{AudioBundle, DjSystemDesc};
use crate::audio::audio::Music;
use state::start::StartScreen;
use amethyst::window::{DisplayConfig};
use amethyst::winit::Icon;
use crate::persistence::Settings;

#[macro_use]
extern crate serde;

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let mut display_config = DisplayConfig::default();
    let settings = Settings::read_or_default();
    display_config.loaded_icon = Some(Icon::from_path("assets/texture/logo.png")?);
    display_config.dimensions = Some((settings.window_settings.arena_width() as u32, settings.window_settings.arena_height() as u32));
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(app_root.join("config").join("bindings.ron"))?;
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(ball::ball_system::MoveBallsSystem, "ball_system", &[])
        .with(timer::timer_system::TimerSystem, "timer_system", &[])
        .with(taunt::taunt_system::TauntSystem, "taunt_system", &[])
        .with(ball::trajectory_system::TrajectorySystem, "trajectory_system", &["ball_system"])
        .with(paddle::paddle::PaddleSystem, "paddle_system", &["input_system", "trajectory_system"])
        .with(ball::bounce_system::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(winner::winner::WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, StartScreen::new(settings), game_data)?;
    game.run();
    Ok(())
}
