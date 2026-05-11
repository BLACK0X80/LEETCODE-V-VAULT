impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.as_bytes();
        let key  = key.as_bytes();
        let (rn, kn) = (ring.len(), key.len());

        let mut pos = vec![vec![]; 26];
        for (i, &c) in ring.iter().enumerate() {
            pos[(c - b'a') as usize].push(i);
        }

        let mut dp = vec![i32::MAX; rn];
        dp[0] = 0;

        for i in 0..kn {
            let mut ndp = vec![i32::MAX; rn];
            for &j in &pos[(key[i] - b'a') as usize] {
                for (prev, &cost) in dp.iter().enumerate() {
                    if cost == i32::MAX { continue; }
                    let diff = (j as i32 - prev as i32).abs();
                    let rotate = diff.min(rn as i32 - diff);
                    ndp[j] = ndp[j].min(cost + rotate + 1);
                }
            }
            dp = ndp;
        }

        *dp.iter().filter(|&&x| x != i32::MAX).min().unwrap()
    }
}