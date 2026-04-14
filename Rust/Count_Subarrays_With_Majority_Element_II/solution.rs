impl Solution {
    pub fn count_majority_subarrays(black_nums: Vec<i32>, black_target: i32) -> i64 {
        let black_n = black_nums.len();
        let mut black_tree = vec![0i64; 2 * black_n + 2];
        let mut black_sum = black_n;
        let mut black_ans = 0i64;
        
        let black_sz = black_tree.len();
        let mut black_idx = black_sum + 1;
        while black_idx < black_sz { black_tree[black_idx] += 1; black_idx += black_idx & (!black_idx + 1); }

        for x in black_nums {
            if x == black_target { black_sum += 1; } else { black_sum -= 1; }
            
            let mut black_tmp_q = black_sum;
            while black_tmp_q > 0 { black_ans += black_tree[black_tmp_q]; black_tmp_q -= black_tmp_q & (!black_tmp_q + 1); }
            
            let mut black_tmp_u = black_sum + 1;
            while black_tmp_u < black_sz { black_tree[black_tmp_u] += 1; black_tmp_u += black_tmp_u & (!black_tmp_u + 1); }
        }
        black_ans
    }
}