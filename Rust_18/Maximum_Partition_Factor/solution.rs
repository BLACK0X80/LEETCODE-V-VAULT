impl Solution {
    pub fn max_partition_factor(points: Vec<Vec<i32>>) -> i32 {
        let black_n = points.len();
        if black_n == 2 { return 0; }
        
        let mut black_low = 0;
        let mut black_high = 400_000_000;
        let mut black_ans = 0;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::black_check(black_mid, &points) {
                black_ans = black_mid;
                black_low = black_mid + 1;
            } else {
                black_high = black_mid - 1;
            }
        }
        black_ans
    }

    fn black_check(black_dist: i32, black_pts: &Vec<Vec<i32>>) -> bool {
        let black_n = black_pts.len();
        let mut black_adj = vec![vec![]; black_n];
        for i in 0..black_n {
            for j in i + 1..black_n {
                let black_d = (black_pts[i][0] - black_pts[j][0]).abs() + (black_pts[i][1] - black_pts[j][1]).abs();
                if black_d < black_dist {
                    black_adj[i].push(j);
                    black_adj[j].push(i);
                }
            }
        }
        
        let mut black_color = vec![0; black_n];
        for i in 0..black_n {
            if black_color[i] == 0 {
                let mut black_stack = vec![i];
                black_color[i] = 1;
                while let Some(u) = black_stack.pop() {
                    for &v in &black_adj[u] {
                        if black_color[v] == 0 {
                            black_color[v] = 3 - black_color[u];
                            black_stack.push(v);
                        } else if black_color[v] == black_color[u] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}