use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Debug)]
pub struct Fighter {
    attack: u32,
    defense: u32
}

impl Component for Fighter {
    type Storage = DenseVecStorage<Self>;
}

impl Fighter {
    pub fn new(attack: u32, defense: u32) -> Self {
        Fighter {
            attack,
            defense
        }
    }

    pub fn attack(&self) -> u32 {
        self.attack
    }

    pub fn defense(&self) -> u32 {
        self.defense
    }
}
