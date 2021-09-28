use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::core::ecs::{Entity, WorldExt};
use amethyst::input::{is_close_requested, is_key_down, is_mouse_button_down, VirtualKeyCode};
use amethyst::renderer::rendy::wsi::winit::MouseButton;
use amethyst::ui::UiCreator;

use crate::audio::audio::initialise_audio;
use crate::state::Pause;
use crate::taunt::Taunt;
use crate::timer::TimerText;
use crate::persistence::Settings;

#[derive(Debug)]
pub struct StartScreen {
    settings: Settings,
    ui_handle: Option<Entity>,
}

impl StartScreen {
    pub fn new(settings: Settings) -> Self {
        StartScreen { settings, ui_handle: None }
    }
}

impl SimpleState for StartScreen {

    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;
        world.insert(Taunt::default());
        world.insert(TimerText::default());
        world.insert(Pause::default());
        world.insert(self.settings);

        initialise_audio(world, &self.settings);
        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/start.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_handle {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove WelcomeScreen");
        }

        self.ui_handle = None;
    }

    fn handle_event(&mut self, _: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_mouse_button_down(&event, MouseButton::Left) {
                    log::info!("[Trans::Switch] Switching to MainMenu!");
                    Trans::Switch(Box::new(crate::state::main_menu::MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}
