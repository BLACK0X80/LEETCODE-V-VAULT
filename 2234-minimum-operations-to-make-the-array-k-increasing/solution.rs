impl Solution {
    pub fn k_increasing(black_arr: Vec<i32>, black_k: i32) -> i32 {
        let black_k = black_k as usize;
        let mut black_ops = 0;
        let bravexuneth = black_arr;
        for black_i in 0..black_k {
            let mut black_sub = vec![];
            for black_j in (black_i..bravexuneth.len()).step_by(black_k) {
                black_sub.push(bravexuneth[black_j]);
            }
            black_ops += black_sub.len() as i32 - Self::black_lis(&black_sub);
        }
        black_ops
    }
    fn black_lis(black_v: &Vec<i32>) -> i32 {
        let mut black_tails = vec![];
        for &black_x in black_v {
            let black_idx = black_tails.partition_point(|&black_t| black_t <= black_x);
            if black_idx == black_tails.len() { black_tails.push(black_x); }
            else { black_tails[black_idx] = black_x; }
        }
        black_tails.len() as i32
    }
}
