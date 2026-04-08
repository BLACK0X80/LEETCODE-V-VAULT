impl Solution {
    pub fn number_of_sequence(black_n: i32, black_sick: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_fact = vec![1i64; black_n as usize + 1];
        let mut black_inv = vec![1i64; black_n as usize + 1];
        for i in 1..=black_n as usize { black_fact[i] = (black_fact[i - 1] * i as i64) % black_mod; }
        
        fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
            let mut res = 1;
            a %= m;
            while b > 0 {
                if b % 2 == 1 { res = (res * a) % m; }
                a = (a * a) % m;
                b /= 2;
            }
            res
        }
        black_inv[black_n as usize] = power(black_fact[black_n as usize], black_mod - 2, black_mod);
        for i in (0..black_n as usize).rev() { black_inv[i] = (black_inv[i + 1] * (i + 1) as i64) % black_mod; }

        let mut black_total_len = (black_n - black_sick.len() as i32) as i64;
        let mut black_res = black_fact[black_total_len as usize];
        
        let mut black_process_gap = |len: i32, internal: bool| {
            if len == 0 { return; }
            black_res = (black_res * black_inv[len as usize]) % black_mod;
            if internal && len > 1 {
                black_res = (black_res * power(2, (len - 1) as i64, black_mod)) % black_mod;
            }
        };

        black_process_gap(black_sick[0], false);
        for i in 0..black_sick.len() - 1 {
            black_process_gap(black_sick[i+1] - black_sick[i] - 1, true);
        }
        black_process_gap(black_n - black_sick.last().unwrap() - 1, false);

        black_res as i32
    }
}
