impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let (lx, ly) = (location[0] as f64, location[1] as f64);
        let mut angles: Vec<f64> = Vec::new();
        let mut same = 0;

        for p in &points {
            let (dx, dy) = (p[0] as f64 - lx, p[1] as f64 - ly);
            if dx == 0.0 && dy == 0.0 { same += 1; continue; }
            angles.push(dy.atan2(dx).to_degrees());
        }

        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = angles.len();
        for i in 0..len { angles.push(angles[i] + 360.0); }

        let ang = angle as f64;
        let mut best = 0;
        let mut l = 0;
        for r in 0..angles.len() {
            while angles[r] - angles[l] > ang { l += 1; }
            best = best.max(r - l + 1);
        }

        (best as i32) + same
    }
}
