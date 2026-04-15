impl Solution {
    pub fn can_make_palindrome_queries(black_s: String, black_queries: Vec<Vec<i32>>) -> Vec<bool> {
        let black_b = black_s.as_bytes();
        let black_n = black_b.len();
        let black_m = black_n / 2;
        let mut black_p1 = vec![[0i32; 26]; black_m + 1];
        let mut black_p2 = vec![[0i32; 26]; black_m + 1];
        let mut black_d = vec![0i32; black_m + 1];

        for i in 0..black_m {
            black_p1[i + 1] = black_p1[i];
            black_p2[i + 1] = black_p2[i];
            black_p1[i + 1][(black_b[i] - b'a') as usize] += 1;
            black_p2[i + 1][(black_b[black_n - 1 - i] - b'a') as usize] += 1;
            black_d[i + 1] = black_d[i] + (black_b[i] != black_b[black_n - 1 - i]) as i32;
        }

        let black_get = |p: &[[i32; 26]], l: usize, r: usize| {
            let mut res = [0i32; 26];
            if l <= r { for i in 0..26 { res[i] = p[r + 1][i] - p[l][i]; } }
            res
        };

        black_queries.into_iter().map(|q| {
            let (a, b) = (q[0] as usize, q[1] as usize);
            let (c, d) = (black_n - 1 - q[3] as usize, black_n - 1 - q[2] as usize);
            
            if black_d[a.min(c)] > 0 || black_d[black_m] - black_d[b.max(d) + 1] > 0 { return false; }
            if b < c && black_d[c] - black_d[b + 1] > 0 { return false; }
            if d < a && black_d[a] - black_d[d + 1] > 0 { return false; }

            let mut black_c1 = black_get(&black_p1, a, b);
            let mut black_c2 = black_get(&black_p2, c, d);

            if a < c {
                let fix = black_get(&black_p2, a, (c - 1).min(b));
                for i in 0..26 { black_c1[i] -= fix[i]; if black_c1[i] < 0 { return false; } }
                if c > b + 1 && black_d[c] - black_d[b + 1] > 0 { return false; }
            }
            if b > d {
                let fix = black_get(&black_p2, (d + 1).max(a), b);
                for i in 0..26 { black_c1[i] -= fix[i]; if black_c1[i] < 0 { return false; } }
            }
            if c < a {
                let fix = black_get(&black_p1, c, (a - 1).min(d));
                for i in 0..26 { black_c2[i] -= fix[i]; if black_c2[i] < 0 { return false; } }
                if a > d + 1 && black_d[a] - black_d[d + 1] > 0 { return false; }
            }
            if d > b {
                let fix = black_get(&black_p1, (b + 1).max(c), d);
                for i in 0..26 { black_c2[i] -= fix[i]; if black_c2[i] < 0 { return false; } }
            }

            black_c1 == black_c2
        }).collect()
    }
}