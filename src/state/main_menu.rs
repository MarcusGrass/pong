use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::assets::{Handle, Loader, AssetStorage};
use amethyst::core::ecs::{Entity, WorldExt, World, Builder};
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use amethyst::renderer::{SpriteSheet, Texture, ImageFormat, SpriteRender, Sprite};
use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiFinder};

use crate::state::pong::{Pong};
use crate::state::start::StartScreen;
use crate::taunt::{TauntComponent, Taunt};
use amethyst::core::Transform;
use crate::state::options::OptionState;
use crate::persistence::window::WindowSettings;
use amethyst::core::math::Vector3;
use crate::persistence::Settings;

const BUTTON_START: &str = "start";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_EXIT: &str = "exit";

#[derive(Default)]
pub struct MainMenu {
    sprite_sheet: Option<Handle<SpriteSheet>>,
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_options: Option<Entity>,
    button_exit: Option<Entity>,
}


impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        // create UI from prefab and save the reference.
        let world = data.world;
        let read = world.read_resource::<Settings>();
        let settings = *read;
        drop(read);
        let sprite_sheet = load_sprite_sheet(world);
        self.sprite_sheet.replace(sprite_sheet.clone());

        initialize_taunt(world, sprite_sheet.clone(), settings.window_settings);
        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_options = None;
        self.button_exit = None;
    }

    fn handle_event(&mut self, data: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    Trans::Switch(Box::new(StartScreen::new(*data.world.read_resource::<Settings>())))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                               event_type: UiEventType::Click,
                               target,
                           }) => {
                if Some(target) == self.button_start {
                    log::info!("[Trans::Switch] Switching to Game!");
                    return Trans::Switch(Box::new(Pong::new(self.sprite_sheet.clone().unwrap(), data.world.read_resource::<Settings>().window_settings)));
                }
                if Some(target) == self.button_options {
                    return Trans::Switch(Box::new(OptionState::default()));
                }
                if Some(target) == self.button_exit {
                    return Trans::Quit
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_options.is_none()
            || self.button_exit.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_exit = ui_finder.find(BUTTON_EXIT);
            });
        }

        Trans::None
    }
}


fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };


    {
        let sheet = SpriteSheet {
            texture: texture_handle,
            sprites: vec![
                to_sprite(16, 64, 768, 0),
                to_sprite(25, 25, 1024, 0),
                to_sprite(256, 256, 0, 0),
                to_sprite(256, 256, 256, 0),
                to_sprite(256, 256, 512, 0),
            ]
        };
        let loader = world.read_resource::<Loader>();
        let mut sprite_storage = world.write_resource::<AssetStorage<SpriteSheet>>();
        sprite_storage.insert(sheet.clone());
        loader.load_from_data(sheet, (), &sprite_storage)
    }

}

fn to_sprite(width: u32, height: u32, x: u32, y: u32) -> Sprite {
    Sprite::from_pixel_values(
        1280, 256, width, height as u32, x, y,
        [0f32, 0.0], false, false)
}

fn initialize_taunt(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, window_settings: WindowSettings) {
    world.register::<TauntComponent>();
    let mut taunt_transform = Transform::default();
    taunt_transform.set_translation_xyz(window_settings.arena_width() / 2.0, window_settings.arena_height() - window_settings.taunt_height() / 2.0, -1.0);
    taunt_transform.set_scale(Vector3::new(window_settings.taunt_scale(), window_settings.taunt_scale(), 1.0));
    let sprite = SpriteRender::new(sprite_sheet_handle, 3);
    let taunt = world.create_entity()
        .with(taunt_transform)
        .with(TauntComponent)
        .with(sprite)
        .build();
    world.get_mut::<Taunt>().unwrap().face.replace(taunt);
}
