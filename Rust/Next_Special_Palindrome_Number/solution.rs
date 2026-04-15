impl Solution {
    pub fn special_palindrome(n: i64) -> i64 {
        let mut black_results = Vec::new();
        let black_digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 1..(1 << 9) {
            let mut black_set = Vec::new();
            let (mut black_sum, mut black_odds) = (0, 0);
            for j in 0..9 {
                if (i >> j) & 1 == 1 {
                    black_set.push(black_digits[j]);
                    black_sum += black_digits[j];
                    if black_digits[j] % 2 != 0 { black_odds += 1; }
                }
            }
            
            if black_odds <= 1 && black_sum <= 16 {
                let mut black_half = Vec::new();
                let mut black_mid = None;
                for &d in &black_set {
                    if d % 2 != 0 {
                        black_mid = Some(d);
                        for _ in 0..(d / 2) { black_half.push(d); }
                    } else {
                        for _ in 0..(d / 2) { black_half.push(d); }
                    }
                }
                
                Self::black_permute(&mut black_half, black_mid, &mut black_results);
            }
        }
        black_results.sort_unstable();
        black_results.dedup();
        *black_results.iter().find(|&&x| x > n).unwrap_or(&-1)
    }

    fn black_permute(black_h: &mut Vec<u8>, black_m: Option<u8>, black_res: &mut Vec<i64>) {
        black_h.sort_unstable();
        loop {
            if !black_h.is_empty() && black_h[0] == 0 { if !Self::black_next_p(black_h) { break; } continue; }
            let mut black_f = black_h.clone();
            if let Some(m) = black_m { black_f.push(m); }
            for i in (0..black_h.len()).rev() { black_f.push(black_h[i]); }
            black_res.push(black_f.iter().fold(0i64, |a, &b| a * 10 + b as i64));
            if !Self::black_next_p(black_h) { break; }
        }
    }

    fn black_next_p(black_p: &mut Vec<u8>) -> bool {
        if black_p.len() < 2 { return false; }
        let mut i = black_p.len() - 1;
        while i > 0 && black_p[i-1] >= black_p[i] { i -= 1; }
        if i == 0 { return false; }
        let mut j = black_p.len() - 1;
        while black_p[j] <= black_p[i-1] { j -= 1; }
        black_p.swap(i-1, j);
        black_p[i..].reverse();
        true
    }
}