pub struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Rect {
            x,
            y,
            w,
            h
        }
    }

    pub fn top(&self) -> u32 {
        self.y + self.h as u32 - 1
    }

    pub fn left(&self) -> u32 {
        self.x
    }

    pub fn bottom(&self) -> u32 {
        self.y
    }

    pub fn right(&self) -> u32 {
        self.x + self.w - 1
    }

    pub fn center(&self) -> (u32, u32) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}
