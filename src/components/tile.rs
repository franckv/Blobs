use amethyst::ecs::{Component, DenseVecStorage};

pub struct Tile {
    block: bool,
    transparent: bool
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            block: true,
            transparent: false
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

impl Tile {
    pub fn is_block(&self) -> bool {
        self.block
    }

    pub fn is_transparent(&self) -> bool {
        self.transparent
    }
}
