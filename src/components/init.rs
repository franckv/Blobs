use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Init;
impl Component for Init {
    type Storage = DenseVecStorage<Self>;
}


