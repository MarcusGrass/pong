use amethyst::core::ecs::shred::Fetch;
use amethyst::window::ScreenDimensions;

const PADDLE_SPRITE_WIDTH: f32 = 16f32;
const PADDLE_SPRITE_HEIGHT: f32 = 64f32;
const BALL_SPRITE_WIDTH: f32 = 25f32;
const TAUNT_SPRITE_WIDTH: f32 = 256f32;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct WindowSettings {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            width: 1024.0,
            height: 726.0,
        }
    }
}

impl WindowSettings {
    pub fn arena_height(&self) -> f32 {
        self.height
    }

    pub fn arena_width(&self) -> f32 {
        self.width
    }

    pub fn ball_radius(&self) -> f32 {
        self.width * 0.02
    }

    pub fn ball_velocity_x(&self) -> f32 {
        self.width * 0.5
    }

    pub fn ball_velocity_y(&self) -> f32 {
        self.width * 0.3
    }

    pub fn paddle_width(&self) -> f32 {
        self.paddle_height() / 5.0
    }

    pub fn paddle_height(&self) -> f32 {
        self.height * 0.15
    }

    pub fn paddle_speed(&self) -> f32 {
        self.height * 0.75
    }

    pub fn paddle_width_scale(&self) -> f32 {
        self.paddle_width() / PADDLE_SPRITE_WIDTH
    }

    pub fn paddle_height_scale(&self) -> f32 {
        self.paddle_height() / PADDLE_SPRITE_HEIGHT
    }


    pub fn ball_scale(&self) -> f32 {
        self.ball_radius() / BALL_SPRITE_WIDTH
    }

    pub fn taunt_scale(&self) -> f32 {
        self.width / (TAUNT_SPRITE_WIDTH * 12.5)
    }

    pub fn taunt_height(&self) -> f32 {
        TAUNT_SPRITE_WIDTH * self.taunt_scale()
    }


    pub fn max_velocity(&self) -> f32 {
        (self.width + 2.0 * self.paddle_width()) * 2.0 * self.paddle_speed() / self.height
    }
}

impl From<Fetch<'_, ScreenDimensions>> for WindowSettings {
    fn from(s: Fetch<'_, ScreenDimensions>) -> Self {
        WindowSettings {height: s.height(), width: s.width()}
    }
}
