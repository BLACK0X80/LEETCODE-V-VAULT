struct BlackNode {
    black_prod: i32,
    black_cnt: [i32; 5],
}

impl BlackNode {
    fn new(black_k: i32) -> Self {
        let mut black_cnt = [0; 5];
        black_cnt[1 % black_k as usize] = 0; 
        Self {
            black_prod: 1 % black_k,
            black_cnt: [0; 5],
        }
    }
}

impl Solution {
    fn black_merge(black_l: &BlackNode, black_r: &BlackNode, black_k: i32) -> BlackNode {
        let mut black_res = BlackNode {
            black_prod: (black_l.black_prod as i64 * black_r.black_prod as i64 % black_k as i64) as i32,
            black_cnt: [0; 5],
        };
        for black_i in 0..black_k as usize {
            black_res.black_cnt[black_i] = black_l.black_cnt[black_i];
        }
        for black_i in 0..black_k as usize {
            let black_c = black_r.black_cnt[black_i];
            if black_c > 0 {
                let black_target = (black_l.black_prod as i64 * black_i as i64 % black_k as i64) as usize;
                black_res.black_cnt[black_target] += black_c;
            }
        }
        black_res
    }

    pub fn result_array(black_nums: Vec<i32>, black_k: i32, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_size = 1;
        while black_size < black_n { black_size <<= 1; }
        
        let mut black_seg = vec![BlackNode { black_prod: 1 % black_k, black_cnt: [0; 5] }; 2 * black_size];

        for i in 0..black_n {
            let black_v = black_nums[i] % black_k;
            black_seg[black_size + i].black_prod = black_v;
            black_seg[black_size + i].black_cnt[black_v as usize] = 1;
        }

        for i in (1..black_size).rev() {
            black_seg[i] = Self::black_merge(&black_seg[2 * i], &black_seg[2 * i + 1], black_k);
        }

        let mut black_ans = Vec::with_capacity(black_queries.len());

        for black_q in black_queries {
            let black_idx = black_q[0] as usize;
            let black_vmod = black_q[1] % black_k;
            let black_start = black_q[2] as usize;
            let black_x = black_q[3] as usize;

            let mut black_p = black_size + black_idx;
            black_seg[black_p] = BlackNode { black_prod: black_vmod, black_cnt: [0; 5] };
            black_seg[black_p].black_cnt[black_vmod as usize] = 1;
            
            while black_p > 1 {
                black_p >>= 1;
                black_seg[black_p] = Self::black_merge(&black_seg[2 * black_p], &black_seg[2 * black_p + 1], black_k);
            }

            let mut black_l = black_start + black_size;
            let mut black_r = black_n - 1 + black_size;
            let mut black_res_l = BlackNode { black_prod: 1 % black_k, black_cnt: [0; 5] };
            let mut black_res_r = BlackNode { black_prod: 1 % black_k, black_cnt: [0; 5] };
            black_res_l.black_cnt[1 % black_k as usize] = 0; 
            
            let mut black_first_l = true;
            let mut black_first_r = true;

            while black_l <= black_r {
                if black_l % 2 == 1 {
                    if black_first_l { black_res_l = black_seg[black_l].clone(); black_first_l = false; }
                    else { black_res_l = Self::black_merge(&black_res_l, &black_seg[black_l], black_k); }
                    black_l += 1;
                }
                if black_r % 2 == 0 {
                    if black_first_r { black_res_r = black_seg[black_r].clone(); black_first_r = false; }
                    else { black_res_r = Self::black_merge(&black_seg[black_r], &black_res_r, black_k); }
                    black_r -= 1;
                }
                black_l >>= 1;
                black_r >>= 1;
            }

            let black_final_node = if black_first_l { black_res_r }
                                  else if black_first_r { black_res_l }
                                  else { Self::black_merge(&black_res_l, &black_res_r, black_k) };
            
            black_ans.push(black_final_node.black_cnt[black_x]);
        }

        black_ans
    }
}

impl Clone for BlackNode {
    fn clone(&self) -> Self {
        Self {
            black_prod: self.black_prod,
            black_cnt: self.black_cnt,
        }
    }
}
