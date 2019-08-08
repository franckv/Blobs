use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Player;
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}


