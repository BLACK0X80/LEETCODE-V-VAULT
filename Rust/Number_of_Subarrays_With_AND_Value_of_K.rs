impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let (mut black_res, mut black_l1, mut black_l2) = (0i64, 0, 0);
        let (mut black_c1, mut black_c2) = (vec![0; 32], vec![0; 32]);
        for black_r in 0..black_nums.len() {
            for black_i in 0..32 { if (black_nums[black_r] >> black_i) & 1 == 0 { black_c1[black_i] += 1; black_c2[black_i] += 1; } }
            while black_l1 <= black_r && (0..32).fold(u32::MAX, |black_a, black_i| if black_c1[black_i] == 0 { black_a } else { black_a & !(1 << black_i) }) < black_k as u32 {
                for black_i in 0..32 { if (black_nums[black_l1] >> black_i) & 1 == 0 { black_c1[black_i] -= 1; } }
                black_l1 += 1;
            }
            while black_l2 <= black_r && (0..32).fold(u32::MAX, |black_a, black_i| if black_c2[black_i] == 0 { black_a } else { black_a & !(1 << black_i) }) <= black_k as u32 {
                for black_i in 0..32 { if (black_nums[black_l2] >> black_i) & 1 == 0 { black_c2[black_i] -= 1; } }
                black_l2 += 1;
            }
            black_res += (black_l2 - black_l1) as i64;
        }
        black_res
    }
}