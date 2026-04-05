impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let black_n = nums1.len();
        let mut black_delta = vec![0; black_n];
        let mut black_vals = Vec::with_capacity(black_n * 2);
        
        for i in 0..black_n {
            black_delta[i] = nums1[i] - nums2[i];
            black_vals.push(black_delta[i]);
            black_vals.push(black_delta[i] + diff); // تعديل: نحتاج البحث عن delta[j] + diff
        }
        
        black_vals.sort_unstable();
        black_vals.dedup();

        let mut black_bit = vec![0; black_vals.len() + 1];
        let mut black_res = 0i64;

        for &black_d in &black_delta {
            // نبحث عن عدد العناصر السابقة i حيث delta[i] <= delta[j] + diff
            let black_target = black_d + diff;
            let black_target_idx = black_vals.binary_search(&black_target).unwrap() + 1;
            
            let mut black_q = black_target_idx;
            while black_q > 0 {
                black_res += black_bit[black_q] as i64;
                black_q -= (black_q as i32 & -(black_q as i32)) as usize;
            }
            
            let black_update_idx = black_vals.binary_search(&black_d).unwrap() + 1;
            let mut black_u = black_update_idx;
            while black_u < black_bit.len() {
                black_bit[black_u] += 1;
                black_u += (black_u as i32 & -(black_u as i32)) as usize;
            }
        }
        black_res
    }
}
