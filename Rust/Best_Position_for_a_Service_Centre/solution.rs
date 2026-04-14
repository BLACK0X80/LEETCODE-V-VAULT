impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let black_dist = |x: f64, y: f64| -> f64 {
            positions.iter().map(|p| {
                let (dx, dy) = (x - p[0] as f64, y - p[1] as f64);
                (dx*dx + dy*dy).sqrt()
            }).sum()
        };

        let mut black_x: f64 = positions.iter().map(|p| p[0] as f64).sum::<f64>() / positions.len() as f64;
        let mut black_y: f64 = positions.iter().map(|p| p[1] as f64).sum::<f64>() / positions.len() as f64;
        let mut black_step = 100.0f64;

        while black_step > 1e-7 {
            let black_cur = black_dist(black_x, black_y);
            let mut black_moved = false;
            for &(dx, dy) in &[(0.0,1.0),(0.0,-1.0),(1.0,0.0),(-1.0,0.0)] {
                let (nx, ny) = (black_x + dx * black_step, black_y + dy * black_step);
                if black_dist(nx, ny) < black_cur {
                    black_x = nx; black_y = ny;
                    black_moved = true;
                    break;
                }
            }
            if !black_moved { black_step *= 0.5; }
        }
        black_dist(black_x, black_y)
    }
}