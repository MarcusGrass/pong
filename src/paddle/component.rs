use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side, height: f32, width: f32) -> Paddle {
        Paddle {
            side,
            width,
            height,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
