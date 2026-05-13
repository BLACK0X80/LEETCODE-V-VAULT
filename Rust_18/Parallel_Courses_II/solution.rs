impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize; let k = k as usize;
        let mut black_pre = vec![0usize; n];
        for r in &relations { black_pre[r[1] as usize-1] |= 1 << (r[0] as usize-1); }
        let mut black_dp = vec![i32::MAX; 1<<n];
        black_dp[0] = 0;
        for mask in 0..(1<<n) {
            if black_dp[mask] == i32::MAX { continue; }
            let black_avail: usize = (0..n).filter(|&i| mask&(1<<i)==0 && black_pre[i]&mask==black_pre[i]).fold(0, |a,i| a|(1<<i));
            let mut sub = black_avail;
            while sub > 0 {
                if (sub as u32).count_ones() as usize <= k {
                    black_dp[mask|sub] = black_dp[mask|sub].min(black_dp[mask]+1);
                }
                sub = (sub-1) & black_avail;
            }
        }
        black_dp[(1<<n)-1]
    }
}