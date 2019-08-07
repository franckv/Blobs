use amethyst::ecs::{Component, DenseVecStorage};

pub struct Init;
impl Component for Init {
    type Storage = DenseVecStorage<Self>;
}


