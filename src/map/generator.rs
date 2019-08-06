use crate::map::Rect;
use crate::components::TileType;
use crate::config::MapConfig;

pub struct Generator {
    width: usize,
    height: usize,
    tiles: Vec<TileType>
}

impl Generator {
    pub fn new(config: &MapConfig) -> Self {
        let mut tiles = Vec::new();

        for _ in 0..config.height {
            for _ in 0..config.width {
                tiles.push(TileType::Full);
            }
        }

        Generator {
            width: config.width,
            height: config.height,
            tiles
        }
    }

    pub fn dig(&mut self, rect: Rect) {
        let (left, right, top, bottom) =
            (rect.left(), rect.right(), rect.top(), rect.bottom());

        for y in left..=right {
            for x in bottom..=top {
                let tile_type = if x == bottom || x == top || y == left || y == right {
                    TileType::Wall
                } else {
                    TileType::None
                };
                self.tiles[x  + y * self.width] = tile_type;
            }
        }
    }

    pub fn tiles(&self) -> &Vec<TileType> {
        &self.tiles
    }

    pub fn tile(&self, x: usize, y: usize) -> TileType {
        self.tiles[x + y * self.width]
    }
}
