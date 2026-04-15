impl Solution {
    pub fn has_same_digits(black_s: String) -> bool {
        let black_n = black_s.len();
        let black_digits: Vec<i32> = black_s.bytes().map(|b| (b - b'0') as i32).collect();
        
        fn ncr_mod(n: usize, r: usize, m: i32) -> i32 {
            if r > n { return 0; }
            let mut num = n;
            let mut den = r;
            let mut count = 0;
            let mut res = 1;
            for i in 0..r {
                let mut curr_num = n - i;
                while curr_num > 0 && curr_num % m as usize == 0 {
                    count += 1;
                    curr_num /= m as usize;
                }
                res = (res * (curr_num % m as usize)) % m as usize;
                let mut curr_den = i + 1;
                while curr_den > 0 && curr_den % m as usize == 0 {
                    count -= 1;
                    curr_den /= m as usize;
                }
                let inv = match m {
                    2 => 1,
                    5 => [0, 1, 3, 2, 4][curr_den % 5],
                    _ => 0
                };
                res = (res * inv) % m as usize;
            }
            if count > 0 { 0 } else { res as i32 }
        }

        
        fn get_comb_vec(n: usize, m: i32) -> Vec<i32> {
            let mut res = vec![1; n + 1];
            let mut count = 0;
            let mut val = 1;
            for r in 1..=n {
                let mut num = n - r + 1;
                while num % m as usize == 0 { count += 1; num /= m as usize; }
                val = (val * (num % m as usize)) % m as usize;
                let mut den = r;
                while den % m as usize == 0 { count -= 1; den /= m as usize; }
                let inv = match m { 2 => 1, 5 => [0, 1, 3, 2, 4][den % 5], _ => 1 };
                val = (val * inv) % m as usize;
                res[r] = if count > 0 { 0 } else { val as i32 };
            }
            res
        }

        let black_c2 = get_comb_vec(black_n - 2, 2);
        let black_c5 = get_comb_vec(black_n - 2, 5);
        let mut black_c10 = vec![0; black_n - 1];
        for i in 0..black_n - 1 {
            let (a, b) = (black_c2[i], black_c5[i]);
            for v in 0..10 { if v % 2 == a && v % 5 == b { black_c10[i] = v; break; } }
        }

        let (mut black_v1, mut black_v2) = (0, 0);
        for i in 0..black_n - 1 {
            black_v1 = (black_v1 + black_digits[i] * black_c10[i]) % 10;
            black_v2 = (black_v2 + black_digits[i + 1] * black_c10[i]) % 10;
        }
        black_v1 == black_v2
    }
}