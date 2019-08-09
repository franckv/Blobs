use amethyst::ecs::{Component, NullStorage};

#[derive(Clone, Debug, Default)]
pub struct Dead;

impl Component for Dead {
    type Storage = NullStorage<Self>;
}
