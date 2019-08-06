use amethyst::ecs::Entity;

use crate::config::MapConfig;

pub struct Map {
    width: usize,
    height: usize,
    tile_size: usize,
    tiles: Vec<Entity>
}

impl Map {
    pub fn new(config: &MapConfig) -> Self {
        Map {
            width: config.width,
            height: config.height,
            tile_size: config.tile_size,
            tiles: Vec::new()
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile_size(&self) -> usize {
        self.tile_size
    }

    pub fn ratio(&self) -> f32 {
        1.0 / self.tile_size as f32
    }

    pub fn tile(&self, x: usize, y: usize) -> Entity {
        let idx = x + y * self.width;

        self.tiles[idx]
    }
}
