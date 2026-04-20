impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let s = word.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut z = vec![0usize; n];
        z[0] = n;
        let (mut l, mut r) = (0, 0);
        for i in 1..n {
            if i < r { z[i] = (r - i).min(z[i - l]); }
            while i + z[i] < n && s[z[i]] == s[i + z[i]] { z[i] += 1; }
            if i + z[i] > r { l = i; r = i + z[i]; }
        }
        let mut t = 1;
        loop {
            let removed = t * k;
            if removed >= n { return t as i32; }
            if z[removed] >= n - removed { return t as i32; }
            t += 1;
        }
    }
}