use amethyst::ecs::Entity;

pub const TILE_SIZE: usize = 16;
pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

pub struct Map {
    width: usize,
    height: usize,
    tile_size: usize,
    tiles: Vec<Entity>
}

impl Default for Map {
    fn default() -> Self {
        Map {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            tile_size: TILE_SIZE,
            tiles: Vec::new()
        }
    }
}

impl Map {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn width_px(&self) -> f32 {
        (self.width * self.tile_size) as f32
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn height_px(&self) -> f32 {
        (self.height * self.tile_size) as f32
    }

    pub fn tile_size(&self) -> usize {
        self.tile_size
    }

    pub fn tile(&self, x: usize, y: usize) -> Entity {
        let idx = x + y * self.width;

        self.tiles[idx]
    }
}
