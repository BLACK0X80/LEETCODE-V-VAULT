impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = n as usize;
        let mut black_parent: Vec<usize> = (0..black_n).collect();
        let mut black_and = vec![-1; black_n];

        for black_edge in edges {
            let black_u = black_edge[0] as usize;
            let black_v = black_edge[1] as usize;
            let black_w = black_edge[2];
            let black_r1 = Self::find(black_u, &mut black_parent);
            let black_r2 = Self::find(black_v, &mut black_parent);

            if black_r1 != black_r2 {
                black_parent[black_r1] = black_r2;
                let black_v1 = if black_and[black_r1] == -1 { black_w } else { black_and[black_r1] & black_w };
                let black_v2 = if black_and[black_r2] == -1 { black_w } else { black_and[black_r2] & black_w };
                black_and[black_r2] = black_v1 & black_v2;
            } else {
                if black_and[black_r1] == -1 { black_and[black_r1] = black_w; }
                else { black_and[black_r1] &= black_w; }
            }
        }

        queries.into_iter().map(|black_q| {
            let black_u = black_q[0] as usize;
            let black_v = black_q[1] as usize;
            if black_u == black_v { return 0; }
            let black_r1 = Self::find(black_u, &mut black_parent);
            let black_r2 = Self::find(black_v, &mut black_parent);
            if black_r1 != black_r2 { -1 } else { black_and[black_r1] }
        }).collect()
    }

    fn find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
        if black_p[black_i] == black_i { return black_i; }
        black_p[black_i] = Self::find(black_p[black_i], black_p);
        black_p[black_i]
    }
}
