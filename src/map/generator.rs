use crate::map::Rect;
use crate::map::Tile;

use rand::{Rng, thread_rng};

pub struct Generator {
    width: u32,
    height: u32,
    min_size: u32,
    max_size: u32,
    max_rooms: u32,
    tiles: Vec<Tile>
}

impl Generator {
    pub fn new(width: u32, height: u32, min_size: u32, max_size: u32, max_rooms: u32) -> Self {
        let mut tiles = Vec::new();

        for _ in 0..height {
            for _ in 0..width {
                tiles.push(Tile::Full);
            }
        }

        Generator {
            width,
            height,
            min_size,
            max_size,
            max_rooms,
            tiles
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn min_size(&self) -> u32 {
        self.min_size
    }

    pub fn max_size(&self) -> u32 {
        self.max_size
    }

    pub fn max_rooms(&self) -> u32 {
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
                        if current == Tile::Full {
                            Tile::Wall
                        } else {
                            current
                        }
                    } else {
                        Tile::Floor
                    };
                let idx = x + y * self.width;
                self.tiles[idx as usize] = tile_type;
            }
        }
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn tile(&self, x: u32, y: u32) -> Tile {
        let idx = x + y * self.width;
        self.tiles[idx as usize]
    }

    pub fn generate(&mut self) -> Vec<Rect> {
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
                rooms.push(r);

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

        rooms
    }
}
