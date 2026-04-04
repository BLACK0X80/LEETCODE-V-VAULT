impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut black = [0i32; 3];
        let mut left = 0;
        let mut ans = 0i32;
        let n = s.len();
        for right in 0..n {
            black[(s[right] - b'a') as usize] += 1;
            while black[0] > 0 && black[1] > 0 && black[2] > 0 {
                ans += (n - right) as i32;
                black[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
        }
        ans
    }
}
