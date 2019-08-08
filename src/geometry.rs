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

pub fn draw_circle<F>(x: i32, y: i32, radius: i32,
                  width: i32, height: i32,
                  mut f: F) where F: FnMut(i32, i32) -> bool {
    let left = (x - radius).max(0);
    let right = (x + radius).min(width);
    let top = (y + radius).min(height);
    let bottom = (y - radius).max(0);

    let mut stop = false;
    for i in bottom..=top {
        if !f(left, i) || !f(right, i) {
            stop = true;
            break;
        }
    }

    if !stop {
        for i in left..=right {
            if !f(i, bottom) || !f(i, top) {
                break;
            }
        }
    }
}


pub fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn diagonal_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs().max((y1 - y2).abs())
}

pub fn circle_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (((x1 - x2) * (x1 - x2)) + ((y1 - y2) * (y1 - y2))).sqrt()
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
