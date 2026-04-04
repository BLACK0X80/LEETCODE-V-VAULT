impl Solution {
    pub fn sum_scores(s: String) -> i64 {
        let b = s.as_bytes();
        let n = b.len();
        let mut z = vec![0usize; n];
        z[0] = n;
        let (mut l, mut r) = (0, 0);
        for i in 1..n {
            if i < r { z[i] = (r - i).min(z[i - l]); }
            while i + z[i] < n && b[z[i]] == b[i + z[i]] { z[i] += 1; }
            if i + z[i] > r { l = i; r = i + z[i]; }
        }
        z.iter().map(|&x| x as i64).sum()
    }
}
