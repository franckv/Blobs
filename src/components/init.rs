use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Init;
impl Component for Init {
    type Storage = NullStorage<Self>;
}


