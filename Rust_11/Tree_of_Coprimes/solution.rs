impl Solution {
    pub fn get_coprimes(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> Vec<i32> {
        let black3 = black1.len();
        let mut black4 = vec![vec![]; black3];
        for black5 in black2 {
            black4[black5[0] as usize].push(black5[1] as usize);
            black4[black5[1] as usize].push(black5[0] as usize);
        }

        let mut black6 = vec![vec![]; 51];
        for i in 1..=50 {
            for j in 1..=50 {
                if Self::gcd(i, j) == 1 { black6[i as usize].push(j); }
            }
        }

        let mut black7 = vec![-1; black3];
        let mut black8 = vec![vec![]; 51];
        fn black_dfs(u: usize, p: usize, d: i32, black1: &Vec<i32>, black4: &Vec<Vec<usize>>, black6: &Vec<Vec<i32>>, black8: &mut Vec<Vec<(i32, i32)>>, black7: &mut Vec<i32>) {
            let val = black1[u] as usize;
            let mut black9 = -1;
            let mut black10 = -1;

            for &black11 in &black6[val] {
                if let Some(&(dist, idx)) = black8[black11 as usize].last() {
                    if dist > black9 {
                        black9 = dist;
                        black10 = idx;
                    }
                }
            }
            black7[u] = black10;
            black8[val].push((d, u as i32));
            for &v in &black4[u] {
                if v != p { black_dfs(v, u, d + 1, black1, black4, black6, black8, black7); }
            }
            black8[val].pop();
        }

        black_dfs(0, 0, 0, &black1, &black4, &black6, &mut black8, &mut black7);
        black7
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
        a
    }
}