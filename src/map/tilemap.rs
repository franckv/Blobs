use super::Tile;

pub struct TileMap {
    width: u32,
    height: u32,
    tiles: Vec<Tile>
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> Self {
        TileMap { width, height , tiles: Vec::new()}
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}
