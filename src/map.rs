
pub const TILE_SIZE: i32 = 16;
pub const MAP_WIDTH: i32 = 32;
pub const MAP_HEIGHT: i32 = 32;

pub struct Map {
    width: i32,
    height: i32,
    tile_size: i32
}

impl Default for Map {
    fn default() -> Self {
        Map {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            tile_size: TILE_SIZE
        }
    }
}

impl Map {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn width_px(&self) -> f32 {
        (self.width * self.tile_size) as f32
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn height_px(&self) -> f32 {
        (self.height * self.tile_size) as f32
    }

    pub fn tile_size(&self) -> i32 {
        self.tile_size
    }
}
