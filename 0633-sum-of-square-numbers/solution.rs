impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let (mut l, mut r) = (0i64, (c as i64).isqrt());
        while l <= r {
            let s = l * l + r * r;
            if s == c as i64 { return true; }
            else if s < c as i64 { l += 1; } else { r -= 1; }
        }
        false
    }
}
