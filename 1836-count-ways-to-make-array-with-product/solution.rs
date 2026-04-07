impl Solution {
    pub fn ways_to_fill_array(black1: Vec<Vec<i32>>) -> Vec<i32> {
        let black2 = 1_000_000_007i64;
        let mut black3 = vec![];
        
        // Precompute combinations
        let mut black4 = vec![vec![0i64; 15]; 10015];
        for i in 0..10015 {
            black4[i][0] = 1;
            for j in 1..=i.min(14) {
                black4[i][j] = (black4[i-1][j-1] + black4[i-1][j]) % black2;
            }
        }

        for q in black1 {
            let n = q[0] as usize;
            let mut k = q[1];
            let mut res = 1i64;
            let mut d = 2;
            while d * d <= k {
                if k % d == 0 {
                    let mut e = 0;
                    while k % d == 0 { e += 1; k /= d; }
                    res = (res * black4[n + e - 1][e]) % black2;
                }
                d += 1;
            }
            if k > 1 { res = (res * n as i64) % black2; }
            black3.push(res as i32);
        }
        black3
    }
}
