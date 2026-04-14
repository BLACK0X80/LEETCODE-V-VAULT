impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        if k == 1 { return 0; }
        let mut black_pair_weights = Vec::with_capacity(n - 1);
        for i in 0..n - 1 {
            black_pair_weights.push(weights[i] as i64 + weights[i+1] as i64);
        }
        
        black_pair_weights.sort_unstable();
        
        let mut black_min_score = 0i64;
        let mut black_max_score = 0i64;
        let m = (k - 1) as usize;
        
        for i in 0..m {
            black_min_score += black_pair_weights[i];
            black_max_score += black_pair_weights[n - 2 - i];
        }
        
        black_max_score - black_min_score
    }
}