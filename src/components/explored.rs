use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Default)]
pub struct Explored;

impl Component for Explored {
    type Storage = DenseVecStorage<Self>;
}
