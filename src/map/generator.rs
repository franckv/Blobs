use crate::map::Rect;
use crate::config::MapConfig;

use rand::{Rng, thread_rng};

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    None,
    Floor,
    Wall,
    Full
}

pub struct Generator {
    width: usize,
    height: usize,
    min_size: usize,
    max_size: usize,
    max_rooms: usize,
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
            min_size: config.min_size,
            max_size: config.max_size,
            max_rooms: config.max_rooms,
            tiles
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn min_size(&self) -> usize {
        self.min_size
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn max_rooms(&self) -> usize {
        self.max_rooms
    }

    pub fn dig(&mut self, rect: &Rect, border: bool) {
        let (left, right, top, bottom) =
            (rect.left(), rect.right(), rect.top(), rect.bottom());

        for y in bottom..=top {
            for x in left..=right {
                let current = self.tile(x, y);
                let tile_type =
                    if border && (x == left || x == right || y == bottom || y == top) {
                        if current == TileType::Full {
                            TileType::Wall
                        } else {
                            current
                        }
                    } else {
                        TileType::Floor
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

    pub fn generate(&mut self) -> (usize, usize) {
        let mut rng = thread_rng();

        let mut rooms = Vec::<Rect>::new();
        for _ in 0..self.max_rooms {
            let w = rng.gen_range(self.min_size, self.max_size + 1);
            let h = rng.gen_range(self.min_size, self.max_size + 1);
            let x = rng.gen_range(0, self.width - w - 1);
            let y = rng.gen_range(0, self.height - h - 1);


            let r = Rect::new(x, y, w, h);
            let mut intersect = false;
            for o in rooms.iter() {
                if r.intersect(&o) {
                    intersect = true;
                }
            }

            if !intersect {
                self.dig(&r, true);
                let (center_x, center_y) = r.center();
                rooms.push(r);
                println!("{}/{} [{}/{}] ({}/{})", x, y, w, h, center_x, center_y);

                let n = rooms.len();
                if n > 1 {
                    let (x1, y1) = &rooms[n - 2].center();
                    let (x2, y2) = &rooms[n - 1].center();

                    let flip = rng.gen_range(0, 2);
                    if flip == 0 {
                        // horizontal tunnel from x1, y1 to x2, y1
                        // vertical tunnel from x2, y1 to x2, y2
                        if x1 < x2 {
                            let t = Rect::new(*x1 - 1, *y1 - 1, *x2 - *x1 + 3, 3);
                            self.dig(&t, true);
                        } else {
                            let t = Rect::new(*x2 - 1, *y1 - 1, *x1 - *x2 + 3, 3);
                            self.dig(&t, true);
                        }

                        if y1 < y2 {
                            let t = Rect::new(*x2 - 1, *y1 - 1, 3, *y2 - *y1 + 3);
                            self.dig(&t, true);
                        } else {
                            let t = Rect::new(*x2 - 1, *y2 - 1, 3, *y1 - *y2 + 3);
                            self.dig(&t, true);
                        }
                    } else {
                        // vertical tunnel from x1, y1 to x1, y2
                        // horizontal tunnel from x1, y2 to x2, y2
                        if y1 < y2 {
                            let t = Rect::new(*x1 - 1, *y1 - 1, 3, *y2 - *y1 + 3);
                            self.dig(&t, true);
                        } else {
                            let t = Rect::new(*x1 - 1, *y2 - 1, 3, *y1 - *y2 + 3);
                            self.dig(&t, true);
                        }

                        if x1 < x2 {
                            let t = Rect::new(*x1 - 1, *y2 - 1, *x2 - *x1 + 3, 3);
                            self.dig(&t, true);
                        } else {
                            let t = Rect::new(*x2 - 1, *y2 - 1, *x1 - *x2 + 3, 3);
                            self.dig(&t, true);
                        }
                    }
                }
            }
        }

        rooms[0].center()
    }
}
