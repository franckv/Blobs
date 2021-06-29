#[derive(Copy, Clone)]
pub enum Distance {
    Manhattan,
    Diagonal,
    Circle
}

pub fn draw_line<F>(x1: i32, y1:i32, x2: i32, y2: i32, mut f:F)
    where F: FnMut(i32, i32) -> bool {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 {1} else {-1};
        let sy = if y1 < y2 {1} else {-1};

        let mut err = if dx > dy {dx / 2} else {-dy / 2};

        let (mut x, mut y) = (x1, y1);

        if !f(x, y) {return};
        while x != x2 || y != y2 {
            let err2 = err;

            if err2 > -dx {
                err -= dy;
                x += sx;
                if !f(x, y) {return};
            }

            if err2 < dy {
                err += dx;
                y += sy;
                if !f(x, y) {return};
            }
        }
    }

pub fn draw_circle_algo<F>(x: i32, y: i32, radius: i32,
                  _width: i32, _height: i32, algo: Distance,
                  mut f: F) where F: FnMut(i32, i32) -> bool {

    let mut dx = 0;
    let mut dy = radius;

    while dx <= dy {
        if !f(x + dy, y + dx) {return};
        if !f(x + dx, y + dy) {return};
        if !f(x - dx, y + dy) {return};
        if !f(x - dy, y + dx) {return};
        if !f(x - dy, y - dx) {return};
        if !f(x - dx, y - dy) {return};
        if !f(x + dx, y - dy) {return};
        if !f(x + dy, y - dx) {return};

        let r1 = distance(x, y, x + dx + 1, y + dy, algo);
        let r2 = distance(x, y, x + dx + 1, y + dy - 1, algo);

        if (r1 - radius as f32).abs() < (r2 - radius as f32).abs() {
            dx += 1;
        } else {
            dx += 1;
            dy -= 1;
        }
    }
}

pub fn draw_circle<F>(x: i32, y: i32, radius: i32,
                  _width: i32, _height: i32,
                  mut f: F) where F: FnMut(i32, i32) -> bool {

    let mut dx = 0;
    let mut dy = radius;

    let mut d = 3 - 2 * radius;

    while dx <= dy {
        if !f(x + dy, y + dx) {return};
        if !f(x + dx, y + dy) {return};
        if !f(x - dx, y + dy) {return};
        if !f(x - dy, y + dx) {return};
        if !f(x - dy, y - dx) {return};
        if !f(x - dx, y - dy) {return};
        if !f(x + dx, y - dy) {return};
        if !f(x + dy, y - dx) {return};

        if d < 0 {
            d = d + 4 * dx + 6;
            dx += 1;
        } else {
            d = d + 4 * (dx - dy) + 10;
            dx += 1;
            dy -= 1;
        }
    }
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32, algo: Distance) -> f32 {
    match algo {
        Distance::Manhattan => (x1 - x2).abs() as f32 + (y1 - y2).abs() as f32,
        Distance::Diagonal => (x1 - x2).abs().max((y1 - y2).abs()) as f32,
        Distance::Circle => (((x1 - x2) * (x1 - x2)) as f32 +
                             ((y1 - y2) * (y1 - y2)) as f32).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_draw_line() {
        let tests = [
            [[5, 5], [10, 10]],
            [[0, 0], [4, 12]],
            [[10, 5], [2, 2]],
            [[10, 5], [2, 10]],
            [[-10, 5], [2, 2]]
        ];

        for test in tests.iter() {
            let start = test[0];
            let end = test[1];
            let mut results = Vec::new();

            draw_line(start[0], start[1], end[0], end[1], |x, y| {
                results.push([x, y]);
                true
            });

            println!("{:?}", results);
            assert_eq!(Some(&end), results.last());
        }
    }
}
