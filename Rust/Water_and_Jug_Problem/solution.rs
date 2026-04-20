impl Solution {
    pub fn can_measure_water(black_x: i32, black_y: i32, black_t: i32) -> bool {
        black_t == 0 || (black_t <= black_x + black_y && black_t % { let (mut black_a, mut black_b) = (black_x, black_y); while black_b != 0 { black_a %= black_b; std::mem::swap(&mut black_a, &mut black_b); } black_a } == 0)
    }
}