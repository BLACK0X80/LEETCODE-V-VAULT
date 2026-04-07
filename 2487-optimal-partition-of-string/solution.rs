impl Solution {
    pub fn partition_string(black: String) -> i32 {
        let (mut mask, mut ans) = (0u32, 1);
        for b in black.bytes().map(|b| 1 << (b - b'a')) {
            if mask & b != 0 { ans += 1; mask = 0; }
            mask |= b;
        }
        ans
    }
}
