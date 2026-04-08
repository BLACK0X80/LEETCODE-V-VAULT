impl Solution {
    pub fn permute(black_n: i32, mut black_k: i64) -> Vec<i32> {
        let mut black_f = vec![1i128; 101];
        for i in 1..=100 {
            black_f[i] = black_f[i - 1] * i as i128;
            if black_f[i] > 2e15 as i128 { black_f[i] = 2e15 as i128; }
        }

        let black_count = |o: i32, e: i32, next_is_odd: bool| -> i64 {
            if o < 0 || e < 0 { return 0; }
            if o == 0 && e == 0 { return 1; }
            if next_is_odd {
                if o != e && o != e + 1 { return 0; }
            } else {
                if e != o && e != o + 1 { return 0; }
            }
            let res = black_f[o as usize] * black_f[e as usize];
            if res > 2e15 as i128 { 2e15 as i64 } else { res as i64 }
        };

        let mut black_res = Vec::new();
        let mut black_used = vec![false; (black_n + 1) as usize];
        let (mut black_o_rem, mut black_e_rem) = (0, 0);
        for i in 1..=black_n {
            if i % 2 == 1 { black_o_rem += 1; } else { black_e_rem += 1; }
        }

        for _ in 0..black_n {
            let mut black_found = false;
            for v in 1..=black_n {
                if black_used[v as usize] { continue; }
                if !black_res.is_empty() && (*black_res.last().unwrap() % 2 == v % 2) { continue; }

                let (no, ne) = if v % 2 == 1 { (black_o_rem - 1, black_e_rem) } else { (black_o_rem, black_e_rem - 1) };
                let ways = black_count(no, ne, v % 2 == 0);

                if black_k <= ways {
                    black_res.push(v);
                    black_used[v as usize] = true;
                    black_o_rem = no;
                    black_e_rem = ne;
                    black_found = true;
                    break;
                } else {
                    black_k -= ways;
                }
            }
            if !black_found { return vec![]; }
        }
        black_res
    }
}
