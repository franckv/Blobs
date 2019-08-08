use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Mob;
impl Component for Mob {
    type Storage = NullStorage<Self>;
}


