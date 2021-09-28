pub mod taunt_system;

use amethyst::core::ecs::{Entity, Component, DenseVecStorage};

#[derive(Default)]
pub struct TauntComponent;

impl Component for TauntComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Taunt {
    pub(crate) face: Option<Entity>,
}
