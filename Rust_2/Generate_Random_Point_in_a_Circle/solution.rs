use std::cell::Cell;

struct Solution {
    radius: f64,
    x: f64,
    y: f64,
    rng: Cell<u64>,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution { radius, x: x_center, y: y_center, rng: Cell::new(98765431) }
    }

    fn next_f64(&self) -> f64 {
        let mut s = self.rng.get();
        s ^= s << 13; s ^= s >> 7; s ^= s << 17;
        self.rng.set(s);
        s as f64 / u64::MAX as f64
    }

    fn rand_point(&self) -> Vec<f64> {
        let r = self.radius * self.next_f64().sqrt();
        let theta = 2.0 * std::f64::consts::PI * self.next_f64();
        vec![self.x + r * theta.cos(), self.y + r * theta.sin()]
    }
}