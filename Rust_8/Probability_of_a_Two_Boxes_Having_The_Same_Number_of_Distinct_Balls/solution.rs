impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let black_n: i32 = balls.iter().sum::<i32>() / 2;
        let mut black_fact = vec![1.0f64; 50];
        for i in 1..50 { black_fact[i] = black_fact[i-1] * i as f64; }

        fn black_dfs(
            idx: usize, balls: &[i32], fact: &[f64],
            n: i32, left1: i32, d1: i32, d2: i32, ways: f64,
            good: &mut f64, total: &mut f64
        ) {
            if idx == balls.len() {
                if left1 == 0 {
                    *total += ways;
                    if d1 == d2 { *good += ways; }
                }
                return;
            }
            let b = balls[idx];
            for take in 0..=b {
                let rest = b - take;
                if take > left1 { break; }
                let new_ways = ways * fact[b as usize] / fact[take as usize] / fact[rest as usize];
                let nd1 = d1 + if take > 0 { 1 } else { 0 };
                let nd2 = d2 + if rest > 0 { 1 } else { 0 };
                black_dfs(idx+1, balls, fact, n, left1-take, nd1, nd2, new_ways, good, total);
            }
        }

        let mut black_good = 0.0f64;
        let mut black_total = 0.0f64;
        black_dfs(0, &balls, &black_fact, black_n, black_n, 0, 0, 1.0, &mut black_good, &mut black_total);
        black_good / black_total
    }
}