pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect {
            x,
            y,
            w,
            h
        }
    }

    pub fn top(&self) -> usize {
        self.y + self.h - 1
    }

    pub fn left(&self) -> usize {
        self.x
    }

    pub fn bottom(&self) -> usize {
        self.y
    }

    pub fn right(&self) -> usize {
        self.x + self.w - 1
    }

    pub fn center(&self) -> (usize, usize) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h  > other.y
    }
}
