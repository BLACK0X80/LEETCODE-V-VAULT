impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (a, b) = (str1.as_bytes(), str2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0u16; n+1]; m+1];

        for i in 1..=m { dp[i][0] = i as u16; }
        for j in 1..=n { dp[0][j] = j as u16; }

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if a[i-1] == b[j-1] {
                    dp[i-1][j-1] + 1
                } else {
                    dp[i-1][j].min(dp[i][j-1]) + 1
                };
            }
        }

        let mut res = Vec::new();
        let (mut i, mut j) = (m, n);
        while i > 0 && j > 0 {
            if a[i-1] == b[j-1] {
                res.push(a[i-1]);
                i -= 1; j -= 1;
            } else if dp[i-1][j] < dp[i][j-1] {
                res.push(a[i-1]);
                i -= 1;
            } else {
                res.push(b[j-1]);
                j -= 1;
            }
        }
        while i > 0 { res.push(a[i-1]); i -= 1; }
        while j > 0 { res.push(b[j-1]); j -= 1; }

        res.reverse();
        String::from_utf8(res).unwrap()
    }
}