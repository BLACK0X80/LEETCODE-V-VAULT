impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut black = [0i32; 26];
        let mut left = 0;
        let mut max_f = 0;
        let mut ans = 0;
        for right in 0..s.len() {
            let idx = (s[right] - b'A') as usize;
            black[idx] += 1;
            if black[idx] > max_f { max_f = black[idx]; }
            if (right - left + 1) as i32 - max_f > k {
                black[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
