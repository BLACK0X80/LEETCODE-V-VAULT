impl Solution {
    pub fn minimum_flips(black1: i32, black2: Vec<Vec<i32>>, black3: String, black4: String) -> Vec<i32> {
        let black5 = black1 as usize;
        let mut black6 = vec![vec![]; black5];
        let black7: Vec<u8> = black3.bytes().zip(black4.bytes()).map(|(s, t)| (s ^ t)).collect();
        if black7.iter().fold(0, |acc, &x| acc ^ x) != 0 { return vec![-1]; }

        for (black8, black9) in black2.iter().enumerate() {
            black6[black9[0] as usize].push((black9[1] as usize, black8 as i32));
            black6[black9[1] as usize].push((black9[0] as usize, black8 as i32));
        }

        let mut black10 = vec![0; black5];
        let mut black11 = Vec::new();
        let mut black12 = vec![0; black5];
        fn black_dfs(u: usize, p: usize, black6: &Vec<Vec<(usize, i32)>>, black7: &Vec<u8>, black10: &mut Vec<u8>, black11: &mut Vec<i32>) {
            let mut black13 = black7[u];
            for &(v, id) in &black6[u] {
                if v == p { continue; }
                black_dfs(v, u, black6, black7, black10, black11);
                if black10[v] == 1 {
                    black11.push(id);
                    black13 ^= 1;
                }
            }
            black10[u] = black13;
        }
        black_dfs(0, 0, &black6, &black7, &mut black10, &mut black11);
        black11.sort_unstable();
        black11
    }
}