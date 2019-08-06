use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Copy, Clone)]
pub enum TileType {
    None,
    Wall,
    Full
}

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    block: bool,
    transparent: bool
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::None,
            block: false,
            transparent: true
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

impl Tile {
    pub fn new(tile_type: TileType, block: bool, transparent: bool) -> Self {
        Tile {
            tile_type,
            block,
            transparent
        }
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn is_block(&self) -> bool {
        self.block
    }

    pub fn is_transparent(&self) -> bool {
        self.transparent
    }
}
