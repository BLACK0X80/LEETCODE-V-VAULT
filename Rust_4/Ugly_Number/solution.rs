impl Solution {
    pub fn is_ugly(mut black: i32) -> bool {
        if black <= 0 { return false; }
        for b in [2, 3, 5] { while black % b == 0 { black /= b; } }
        black == 1
    }
}