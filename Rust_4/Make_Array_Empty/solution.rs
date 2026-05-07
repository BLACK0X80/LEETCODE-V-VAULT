impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        let black_n = nums.len();
        let mut black_sorted_indices: Vec<usize> = (0..black_n).collect();
        black_sorted_indices.sort_by_key(|&i| nums[i]);

        let mut black_bit = vec![0; black_n + 1];
        for i in 1..=black_n {
            let mut black_idx = i;
            while black_idx <= black_n {
                black_bit[black_idx] += 1;
                black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
            }
        }

        fn black_query(black_bit: &Vec<i32>, mut black_idx: usize) -> i32 {
            let mut black_s = 0;
            while black_idx > 0 {
                black_s += black_bit[black_idx];
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_s
        }

        let mut black_ans = 0i64;
        let mut black_curr = 0;
        for &black_next in &black_sorted_indices {
            if black_next >= black_curr {
                black_ans += (black_query(&black_bit, black_next + 1) - black_query(&black_bit, black_curr)) as i64;
            } else {
                black_ans += (black_query(&black_bit, black_n) - black_query(&black_bit, black_curr) + black_query(&black_bit, black_next + 1)) as i64;
            }
            let mut black_u = black_next + 1;
            while black_u <= black_n {
                black_bit[black_u] -= 1;
                black_u += (black_u as i32 & -(black_u as i32)) as usize;
            }
            black_curr = black_next;
        }
        black_ans
    }
}