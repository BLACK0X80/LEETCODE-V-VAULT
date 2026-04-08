impl Solution {
    pub fn count_coprime(black_mat: Vec<Vec<i32>>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_m = black_mat.len();
        let black_max_val = 150;

        let mut black_mu = vec![0; black_max_val + 1];
        black_mu[1] = 1;
        let mut black_primes = vec![];
        let mut black_is_prime = vec![true; black_max_val + 1];
        for i in 2..=black_max_val {
            if black_is_prime[i] {
                black_primes.push(i);
                black_mu[i] = -1;
            }
            for &p in &black_primes {
                if i * p > black_max_val { break; }
                black_is_prime[i * p] = false;
                if i % p == 0 { black_mu[i * p] = 0; break; }
                else { black_mu[i * p] = -black_mu[i]; }
            }
        }

        let mut black_ans = 0i64;
        for g in 1..=black_max_val {
            if black_mu[g] == 0 { continue; }
            
            let mut black_ways_for_g = 1i64;
            for row in &black_mat {
                let mut black_count_in_row = 0i64;
                for &val in row {
                    if val % g as i32 == 0 { black_count_in_row += 1; }
                }
                black_ways_for_g = (black_ways_for_g * black_count_in_row) % black_mod;
            }

            if black_mu[g] == 1 {
                black_ans = (black_ans + black_ways_for_g) % black_mod;
            } else {
                black_ans = (black_ans - black_ways_for_g + black_mod) % black_mod;
            }
        }

        black_ans as i32
    }
}
