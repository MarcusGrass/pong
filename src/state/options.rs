use amethyst::core::ecs::{Entity, WorldExt, World, WriteStorage};
use amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans};
use amethyst::ui::{UiCreator, UiFinder, UiEvent, UiEventType, UiText};
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use crate::state::main_menu::MainMenu;
use amethyst::audio::AudioSink;
use crate::persistence::{Settings};
use amethyst::shred::ReadExpect;

const MUS_UP_BTN: &str = "mus_up";
const MUS_LBL: &str = "mus_text";
const MUS_DN_BNV: &str = "mus_dn";
const FX_UP_BTN: &str = "fx_up";
const FX_LBL: &str = "fx_text";
const FX_DN_BTN: &str = "fx_dn";
const BACK_BTN: &str = "back";

#[derive(Default)]
pub struct OptionState {
    root: Option<Entity>,
    mus_dn_btn: Option<Entity>,
    mus_lbl: Option<Entity>,
    mus_up_btn: Option<Entity>,
    fx_dn_btn: Option<Entity>,
    fx_lbl: Option<Entity>,
    fx_up_btn: Option<Entity>,
    back_btn: Option<Entity>,
}

impl<'a> SimpleState for OptionState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/options.ron", ())))
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.mus_dn_btn = None;
        self.mus_lbl = None;
        self.mus_up_btn = None;
        self.fx_lbl = None;
        self.fx_up_btn = None;
        self.fx_dn_btn = None;
        self.back_btn = None;
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let world = data.world;
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to Main menu!");
                    Trans::Switch(Box::new(MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                               event_type: UiEventType::Click,
                               target,
                           }) => {
                if self.volume_btn_pushed(Some(target)) {
                    self.update_volume(world, Some(target));
                }
                if Some(target) == self.back_btn {
                    return Trans::Switch(Box::new(MainMenu::default()));
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = data;

        if self.mus_dn_btn.is_none()
            || self.mus_up_btn.is_none()
            || self.mus_lbl.is_none()
            || self.back_btn.is_none()
            || self.fx_up_btn.is_none()
            || self.fx_lbl.is_none()
            || self.fx_dn_btn.is_none()
        {
            world.exec(|(ui_finder, settings, mut write): (UiFinder<'_>, ReadExpect<Settings>, WriteStorage<UiText>)| {
                self.mus_dn_btn = ui_finder.find(MUS_DN_BNV);
                self.mus_lbl = ui_finder.find(MUS_LBL);
                self.mus_up_btn = ui_finder.find(MUS_UP_BTN);
                self.fx_dn_btn = ui_finder.find(FX_DN_BTN);
                self.fx_lbl = ui_finder.find(FX_LBL);
                self.fx_up_btn = ui_finder.find(FX_UP_BTN);
                self.back_btn = ui_finder.find(BACK_BTN);
                if let Some(mus_lbl) = self.mus_lbl {
                    write.get_mut(mus_lbl).unwrap().text = ((settings.audio_settings.music_volume * 10.0) as u8).to_string();
                }
                if let Some(fx_lbl) = self.fx_lbl {
                    write.get_mut(fx_lbl).unwrap().text = ((settings.audio_settings.effects_volume * 10.0) as u8).to_string();
                }
            });


        }

        Trans::None
    }

}
impl OptionState {
    fn update_volume(&self, world: &mut World, target: Option<Entity>) {
        let mus_vol = if target == self.mus_dn_btn || target == self.mus_up_btn {
            if let Some(mus_lbl) = self.mus_lbl {
                let mut write_store = world.write_component::<UiText>();
                if let Some(mus_vol_text) = write_store.get_mut(mus_lbl) {
                    let mut mus: i16 = mus_vol_text.text.parse().unwrap();
                    if target == self.mus_up_btn {
                        mus += 1;
                    } else {
                        mus -= 1;
                    }
                    let mus_vol = clamp(mus, 0, 10);
                    let mut sink = world.write_resource::<AudioSink>();
                    sink.set_volume(mus_vol as f32 / 10.0);
                    mus_vol_text.text = mus_vol.to_string();
                    Some(mus_vol as f32 / 10.0)
                } else { None }
            } else { None }
        } else { None };
        let fx_vol = if target == self.fx_up_btn || target == self.fx_dn_btn {
            if let Some(fx_lbl) = self.fx_lbl {
                let mut write_store = world.write_component::<UiText>();
                if let Some(fx_vol_text) = write_store.get_mut(fx_lbl) {
                    let mut fx: i16 = fx_vol_text.text.parse().unwrap();
                    if target == self.fx_up_btn {
                        fx += 1;
                    } else {
                        fx -= 1;
                    }
                    let fx_vol = clamp(fx, 0, 10);
                    fx_vol_text.text = fx_vol.to_string();
                    Some(fx_vol as f32 / 10.0)
                } else { None }
            } else { None }
        } else { None };

        let mut settings = world.write_resource::<Settings>();
        if let Some(mus) = mus_vol {
            settings.audio_settings.music_volume = mus;
        }
        if let Some(fx) = fx_vol {
            settings.audio_settings.effects_volume = fx;
        }
        if mus_vol.is_some() || fx_vol.is_some() {
            settings.persist_async();
        }
    }

    fn volume_btn_pushed(&self, target: Option<Entity>) -> bool {
        target == self.fx_dn_btn || target == self.fx_up_btn ||
            target == self.mus_up_btn || target == self.mus_dn_btn
    }
}

fn clamp(val: i16, min: u8, max: u8) -> u8 {
    if val < min as i16 {
        min
    } else if val > max as i16 {
        max
    } else {
        val as u8
    }
}
