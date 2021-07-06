pub struct Position {
    x: u32,
    y: u32
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position {
            x,
            y
        }
    }
}