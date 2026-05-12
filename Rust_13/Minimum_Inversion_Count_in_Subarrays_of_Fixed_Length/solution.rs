impl Solution {
    pub fn min_inversion_count(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let (black_n, black_k) = (black_nums.len(), black_k as usize);
        let mut black_sorted = black_nums.clone();
        black_sorted.sort_unstable(); black_sorted.dedup();
        let black_m = black_sorted.len();
        let (mut black_bit, mut black_inv, mut black_ans) = (vec![0i64; black_m + 1], 0i64, i64::MAX);
        for black_i in 0..black_n {
            let black_in = black_sorted.binary_search(&black_nums[black_i]).unwrap() + 1;
            if black_i >= black_k {
                let black_out = black_sorted.binary_search(&black_nums[black_i - black_k]).unwrap() + 1;
                let (mut black_idx, mut black_s) = (black_out - 1, 0);
                while black_idx > 0 { black_s += black_bit[black_idx]; black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize; }
                black_inv -= black_s;
                let mut black_idx = black_out;
                while black_idx <= black_m { black_bit[black_idx] -= 1; black_idx += (black_idx as i32 & -(black_idx as i32)) as usize; }
            }
            let (mut black_idx, mut black_s) = (black_in, 0);
            while black_idx > 0 { black_s += black_bit[black_idx]; black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize; }
            black_inv += (black_i.min(black_k - 1) as i64 - black_s);
            let mut black_idx = black_in;
            while black_idx <= black_m { black_bit[black_idx] += 1; black_idx += (black_idx as i32 & -(black_idx as i32)) as usize; }
            if black_i >= black_k - 1 { black_ans = black_ans.min(black_inv); }
        }
        black_ans
    }
}