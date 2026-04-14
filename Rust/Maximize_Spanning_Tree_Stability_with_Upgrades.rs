impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut black_low = 1;
        let mut black_high = 200001;
        let mut black_ans = -1;

        let mut black_p_init = (0..n as usize).collect::<Vec<_>>();
        for black_e in &edges {
            if black_e[3] == 1 {
                let black_r1 = Self::find(black_e[0] as usize, &mut black_p_init);
                let black_r2 = Self::find(black_e[1] as usize, &mut black_p_init);
                if black_r1 == black_r2 { return -1; }
                black_p_init[black_r1] = black_r2;
            }
        }

        while black_low < black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::check(n, &edges, k, black_mid) {
                black_ans = black_mid;
                black_low = black_mid + 1;
            } else {
                black_high = black_mid;
            }
        }
        black_ans
    }

    fn check(black_n: i32, black_edges: &Vec<Vec<i32>>, black_k: i32, black_target: i32) -> bool {
        let mut black_parent = (0..black_n as usize).collect::<Vec<_>>();
        let mut black_count = 0;
        let mut black_up = 0;

        for black_e in black_edges {
            if black_e[3] == 1 {
                if black_e[2] < black_target { return false; }
                let black_r1 = Self::find(black_e[0] as usize, &mut black_parent);
                let black_r2 = Self::find(black_e[1] as usize, &mut black_parent);
                black_parent[black_r1] = black_r2;
                black_count += 1;
            }
        }

        for black_e in black_edges {
            if black_e[3] == 0 && black_e[2] >= black_target {
                let black_r1 = Self::find(black_e[0] as usize, &mut black_parent);
                let black_r2 = Self::find(black_e[1] as usize, &mut black_parent);
                if black_r1 != black_r2 {
                    black_parent[black_r1] = black_r2;
                    black_count += 1;
                }
            }
        }

        for black_e in black_edges {
            if black_e[3] == 0 && black_e[2] < black_target && black_e[2] * 2 >= black_target {
                if black_up < black_k {
                    let black_r1 = Self::find(black_e[0] as usize, &mut black_parent);
                    let black_r2 = Self::find(black_e[1] as usize, &mut black_parent);
                    if black_r1 != black_r2 {
                        black_parent[black_r1] = black_r2;
                        black_count += 1;
                        black_up += 1;
                    }
                }
            }
        }

        black_count == black_n - 1
    }

    fn find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
        if black_p[black_i] == black_i { return black_i; }
        black_p[black_i] = Self::find(black_p[black_i], black_p);
        black_p[black_i]
    }
}