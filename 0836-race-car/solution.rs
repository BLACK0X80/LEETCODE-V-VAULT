impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let t = target as usize;
        let mut dp = vec![i32::MAX; t + 1];
        dp[0] = 0;

        for i in 1..=t {
            let mut n = 1;
            while (1 << n) < i + 1 { n += 1; }

            if (1 << n) - 1 == i {
                dp[i] = n as i32;
                continue;
            }

            dp[i] = dp[(1 << n) - 1 - i] + n as i32 + 1;

            for m in 0..n {
                let prev = (1 << (n-1)) - 1;
                let back = (1 << m) - 1;
                if prev > back && i > prev - back {
                    dp[i] = dp[i].min(dp[i - prev + back] + (n - 1) as i32 + m as i32 + 2);
                }
            }
        }

        dp[t]
    }
}
