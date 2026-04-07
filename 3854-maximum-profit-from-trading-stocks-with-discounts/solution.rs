impl Solution {
    pub fn max_profit(n: i32, present: Vec<i32>, future: Vec<i32>, hierarchy: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = n as usize;
        let b = budget as usize;
        let mut black1 = vec![vec![]; n];
        for h in hierarchy { black1[(h[0] - 1) as usize].push((h[1] - 1) as usize); }

        let mut black_stack = vec![0];
        let mut black_visited = vec![false; n];
        let mut black_post = Vec::with_capacity(n);
        let mut black_processed = vec![false; n];
        
        while let Some(&u) = black_stack.last() {
            if black_processed[u] {
                black_post.push(u);
                black_stack.pop();
                continue;
            }
            black_processed[u] = true;
            for &v in &black1[u] {
                if !black_processed[v] { black_stack.push(v); }
            }
        }

        let mut dp_no = vec![vec![0; b + 1]; n];
        let mut dp_yes = vec![vec![0; b + 1]; n];

        for &u in &black_post {
            let mut black_tmp_no = vec![0; b + 1];
            let mut black_tmp_yes = vec![0; b + 1];

            for &v in &black1[u] {
                let sub_no = &dp_no[v];
                let sub_yes = &dp_yes[v];
                let sub_cost_v = (present[v] / 2) as usize;
                let sub_prof_v = future[v] - (present[v] / 2);

                for i in (0..=b).rev() {
                    let mut best_v_no = 0;
                    let mut best_v_yes = 0;
                    for j in 0..=i {
                        best_v_no = best_v_no.max(black_tmp_no[i - j] + sub_no[j]);
                        let mut cur_v_yes = sub_no[j];
                        if j >= sub_cost_v {
                            cur_v_yes = cur_v_yes.max(sub_yes[j - sub_cost_v] + sub_prof_v);
                        }
                        best_v_yes = best_v_yes.max(black_tmp_yes[i - j] + cur_v_yes);
                    }
                    black_tmp_no[i] = best_v_no;
                    black_tmp_yes[i] = best_v_yes;
                }
            }

            let cost_u = present[u] as usize;
            let prof_u = future[u] - present[u];
            for i in 0..=b {
                dp_no[u][i] = black_tmp_no[i];
                if i >= cost_u {
                    dp_no[u][i] = dp_no[u][i].max(black_tmp_yes[i - cost_u] + prof_u);
                }
                dp_yes[u][i] = black_tmp_yes[i];
            }
        }

        *dp_no[0].iter().max().unwrap_or(&0)
    }
}
