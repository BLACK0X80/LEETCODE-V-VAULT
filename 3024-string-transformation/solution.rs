impl Solution {
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = s.len();

        fn z_func(s: &[u8]) -> Vec<usize> {
            let n = s.len();
            let mut z = vec![0usize; n];
            let (mut l, mut r) = (0usize, 0usize);
            for i in 1..n {
                if i <= r && z[i - l] <= r - i { z[i] = z[i - l]; }
                else {
                    z[i] = if r > i { r - i + 1 } else { 0 };
                    while i + z[i] < n && s[i + z[i]] == s[z[i]] { z[i] += 1; }
                }
                if i + z[i] - 1 > r { l = i; r = i + z[i] - 1; }
            }
            z
        }

        fn mat_mul(a: &[[i64;2];2], b: &[[i64;2];2]) -> [[i64;2];2] {
            const MOD: i64 = 1_000_000_007;
            let mut c = [[0i64;2];2];
            for i in 0..2 { for j in 0..2 { for l in 0..2 {
                c[i][j] = (c[i][j] + a[i][l] * b[l][j]) % MOD;
            }}}
            c
        }

        fn mat_pow(mut base: [[i64;2];2], mut p: i64) -> [[i64;2];2] {
            let mut res = [[1,0],[0,1]];
            while p > 0 {
                if p & 1 == 1 { res = mat_mul(&res, &base); }
                base = mat_mul(&base, &base);
                p >>= 1;
            }
            res
        }

        let dp = mat_pow([[0, 1], [n as i64 - 1, n as i64 - 2]], k)[0];

        let mut combined = s.as_bytes().to_vec();
        combined.extend_from_slice(t.as_bytes());
        combined.extend_from_slice(t.as_bytes());
        let z = z_func(&combined);

        let mut result = 0i64;
        for i in n..2*n {
            if z[i] >= n {
                let idx = if i - n == 0 { 0 } else { 1 };
                result = (result + dp[idx]) % MOD;
            }
        }

        result as i32
    }
}
