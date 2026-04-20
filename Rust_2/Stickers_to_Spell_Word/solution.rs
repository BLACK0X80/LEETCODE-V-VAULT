impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let t: Vec<u8> = target.bytes().collect();
        let n = t.len();
        let total = 1 << n;
        let mut dp = vec![i32::MAX; total];
        dp[0] = 0;

        for mask in 0..total {
            if dp[mask] == i32::MAX { continue; }
            for sticker in &stickers {
                let mut next = mask;
                let mut freq = [0i32; 26];
                for b in sticker.bytes() { freq[(b - b'a') as usize] += 1; }
                for (i, &c) in t.iter().enumerate() {
                    if next & (1 << i) == 0 {
                        let ci = (c - b'a') as usize;
                        if freq[ci] > 0 {
                            freq[ci] -= 1;
                            next |= 1 << i;
                        }
                    }
                }
                if dp[next] > dp[mask] + 1 {
                    dp[next] = dp[mask] + 1;
                }
            }
        }

        if dp[total - 1] == i32::MAX { -1 } else { dp[total - 1] }
    }
}