use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Debug)]
pub struct Tile {
    block: bool,
    transparent: bool
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            block: false,
            transparent: true
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

impl Tile {
    pub fn new(block: bool, transparent: bool) -> Self {
        Tile {
            block,
            transparent
        }
    }

    pub fn is_block(&self) -> bool {
        self.block
    }

    pub fn is_transparent(&self) -> bool {
        self.transparent
    }
}
