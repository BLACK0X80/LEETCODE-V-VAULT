impl Solution {
    pub fn path_existence_queries(black1: i32, black2: Vec<i32>, black3: i32, black4: Vec<Vec<i32>>) -> Vec<i32> {
        let mut black5 = black2.clone();
        black5.sort_unstable();
        black5.dedup();
        let black6 = black5.len();
        
        let mut black7 = vec![0; black6];
        for i in 1..black6 {
            black7[i] = black7[i - 1] + (black5[i] - black5[i - 1] > black3) as i32;
        }
        
        let mut black8 = vec![vec![0; 18]; black6];
        let mut r = 0;
        for i in 0..black6 {
            while r < black6 && black5[r] <= black5[i] + black3 { r += 1; }
            black8[i][0] = r - 1;
        }
        
        for j in 1..18 {
            for i in 0..black6 {
                black8[i][j] = black8[black8[i][j - 1]][j - 1];
            }
        }

        black4.into_iter().map(|q| {
            let (b_u, b_v) = (q[0] as usize, q[1] as usize);
            if b_u == b_v { return 0; }
            
            let (u_val, v_val) = (black2[b_u], black2[b_v]);
            let (mut x, mut y) = (black5.binary_search(&u_val).unwrap(), black5.binary_search(&v_val).unwrap());
            if x > y { std::mem::swap(&mut x, &mut y); }
            
            if black7[x] != black7[y] { return -1; }
            
            let (mut res, mut curr) = (0, x);
            for j in (0..18).rev() {
                if black8[curr][j] < y {
                    curr = black8[curr][j];
                    res |= 1 << j;
                }
            }
            res + 1
        }).collect()
    }
}
