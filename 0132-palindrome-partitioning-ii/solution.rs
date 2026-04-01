impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut pal = vec![vec![false; n]; n];

        for c in 0..n {
            let mut l = c as i32;
            let mut r = c;
            while l >= 0 && r < n && s[l as usize] == s[r] { pal[l as usize][r] = true; l -= 1; r += 1; }
            l = c as i32; r = c + 1;
            while l >= 0 && r < n && s[l as usize] == s[r] { pal[l as usize][r] = true; l -= 1; r += 1; }
        }

        let mut dp = vec![0i32; n + 1];
        for i in 1..=n {
            dp[i] = i as i32 - 1;
            for j in 0..i {
                if pal[j][i - 1] {
                    dp[i] = dp[i].min(if j == 0 { 0 } else { dp[j] + 1 });
                }
            }
        }

        dp[n]
    }
}
