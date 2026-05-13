impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut a, mut m) = (0, 0);
        for g in gain { a += g; m = m.max(a); }
        m
    }
}