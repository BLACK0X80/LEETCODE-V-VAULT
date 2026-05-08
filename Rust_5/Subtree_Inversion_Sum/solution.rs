impl Solution {
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k_val = k as usize;
        let mut black1 = vec![vec![]; n];
        for e in edges {
            black1[e[0] as usize].push(e[1] as usize);
            black1[e[1] as usize].push(e[0] as usize);
        }

        let mut black2 = vec![vec![]; n];
        let mut black4 = vec![false; n];
        let mut order = Vec::with_capacity(n);
        let mut visit_stack = vec![0];
        
        black4[0] = true;
        while let Some(u) = visit_stack.pop() {
            order.push(u);
            for &v in &black1[u] {
                if !black4[v] {
                    black4[v] = true;
                    black2[u].push(v);
                    visit_stack.push(v);
                }
            }
        }

        let mut dp = vec![vec![[0i64; 2]; k_val]; n];
        for &u in order.iter().rev() {
            for s in 0..k_val {
                for inv in 0..2 {
                    let mut res1 = if inv == 1 { -nums[u] as i64 } else { nums[u] as i64 };
                    let next_s = if s > 0 { s - 1 } else { 0 };
                    for &v in &black2[u] {
                        res1 += dp[v][next_s][inv];
                    }
                    if s == 0 {
                        let mut res2 = if (1 - inv) == 1 { -nums[u] as i64 } else { nums[u] as i64 };
                        for &v in &black2[u] {
                            res2 += dp[v][k_val - 1][1 - inv];
                        }
                        dp[u][s][inv] = res1.max(res2);
                    } else {
                        dp[u][s][inv] = res1;
                    }
                }
            }
        }
        dp[0][0][0]
    }
}