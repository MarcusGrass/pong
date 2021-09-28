pub mod pong;
pub mod start;
pub mod pause;
pub mod main_menu;
pub mod options;

pub struct Pause {
    pub(crate) paused: bool,
}

impl Default for Pause {
    fn default() -> Self {
        Pause { paused: true }
    }
}
