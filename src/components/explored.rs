use amethyst::ecs::{Component, NullStorage};

#[derive(Clone, Debug, Default)]
pub struct Explored;

impl Component for Explored {
    type Storage = NullStorage<Self>;
}
