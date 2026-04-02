impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut last = [-1i64; 26];
        let mut prev = [-1i64; 26];
        let mut res = 0i64;

        for i in 0..n {
            let c = (s[i] - b'A') as usize;
            res += (i as i64 - last[c]) * (n as i64 - i as i64);
            if last[c] >= 0 {
                res -= (last[c] - prev[c]) * (n as i64 - i as i64);
            }
            prev[c] = last[c];
            last[c] = i as i64;
        }

        res as i32
    }
}
