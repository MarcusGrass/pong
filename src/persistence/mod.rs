pub mod window;

use window::WindowSettings;
use amethyst::window::ScreenDimensions;
use amethyst::core::ecs::rayon::spawn_fifo;

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Settings {
    pub window_settings: WindowSettings,
    pub audio_settings: AudioSettings,
}

impl Settings {
    pub fn read_or_default() -> Settings {
        std::fs::read_to_string("config/settings.ron")
            .ok()
            .and_then(|content| ron::from_str::<'_, Settings>(&content).ok())
            .unwrap_or_else(Settings::default)
    }

    pub fn update_window(&mut self, dimensions: &ScreenDimensions) -> bool {
        return if self.window_settings.height != dimensions.height() {
            self.window_settings.height = dimensions.height();
            self.window_settings.width = dimensions.width();
            true
        } else if self.window_settings.width != dimensions.width() {
            self.window_settings.height = dimensions.height();
            self.window_settings.width = dimensions.width();
            true
        } else { false }
    }

    pub fn persist_async(&self) {
        let copy = *self;
        spawn_fifo(move || {
            if let Ok(content) = ron::to_string(&copy) {
                std::fs::write("config/settings.ron", content).unwrap();
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub music_volume: f32,
    pub effects_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            music_volume: 0.1,
            effects_volume: 0.1
        }
    }
}
