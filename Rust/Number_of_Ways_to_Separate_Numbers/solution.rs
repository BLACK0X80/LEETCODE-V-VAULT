impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let num = num.as_bytes();
        let n = num.len();
        let mod_val = 1_000_000_007i64;
        let mut black_lcp = vec![0u16; (n + 1) * (n + 1)];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                let idx = i * (n + 1) + j;
                black_lcp[idx] = if num[i] == num[j] { black_lcp[idx + n + 2] + 1 } else { 0 };
            }
        }
        let mut black_a = vec![0i32; n * n];
        let mut black_b = vec![0i32; n * n];
        for i in (0..n).rev() {
            if num[i] == b'0' { continue; }
            let mut black_sum = 0i64;
            let row_off = i * n;
            for j in (i..n).rev() {
                let idx = row_off + j;
                if j == n - 1 {
                    black_a[idx] = 1;
                } else {
                    let len = j - i + 1;
                    let first = j + 1;
                    let last = first + len;
                    let mut black_cnt = 0i64;
                    if last <= n {
                        let common = black_lcp[i * (n + 1) + first] as usize;
                        if common >= len || num[i + common] <= num[first + common] {
                            black_cnt += black_a[first * n + last - 1] as i64;
                        }
                    }
                    if last < n {
                        black_cnt += black_b[first * n + last] as i64;
                    }
                    black_a[idx] = (black_cnt % mod_val) as i32;
                }
                black_sum += black_a[idx] as i64;
                if black_sum >= mod_val { black_sum -= mod_val; }
                black_b[idx] = black_sum as i32;
            }
        }
        black_b[0]
    }
}