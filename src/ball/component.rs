use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
    pub calculated_impact_y: Option<f32>,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
