impl Solution {
    pub fn count_stable_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = nums.len();
        let (mut black_e, mut black_p, mut black_r) = (vec![0; black_n], vec![0i64; black_n + 1], 0);
        for i in 0..black_n {
            black_r = black_r.max(i);
            while black_r + 1 < black_n && nums[black_r + 1] >= nums[black_r] { black_r += 1; }
            black_e[i] = black_r;
            black_p[i + 1] = black_p[i] + (black_e[i] - i + 1) as i64;
        }
        queries.iter().map(|q| {
            let (l, r) = (q[0] as usize, q[1] as usize);
            let (mut black_low, mut black_high, mut black_split) = (l, r, r + 1);
            while black_low <= black_high {
                let mid = black_low + (black_high - black_low) / 2;
                if black_e[mid] >= r { 
                    black_split = mid; 
                    if mid == 0 { break; }
                    black_high = mid - 1; 
                } else { 
                    black_low = mid + 1; 
                }
            }
            let mut black_res = black_p[black_split.min(black_n)] - black_p[l];
            if black_split <= r {
                let black_len = (r - black_split + 1) as i64;
                black_res += black_len * (black_len + 1) / 2;
            }
            black_res
        }).collect()
    }
}