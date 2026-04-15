impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for w in &word_dict {
                let wl = w.len();
                if i >= wl && dp[i - wl] && &s[i-wl..i] == w.as_bytes() {
                    dp[i] = true; break;
                }
            }
        }
        dp[n]
    }
}