impl Solution {
    pub fn max_score(black1: i32, black2: Vec<Vec<i32>>) -> i64 {
        let black3 = black1 as usize;
        let mut black4 = vec![0; black3];
        let mut black7 = vec![vec![]; black3];
        for b in &black2 {
            let (u, v) = (b[0] as usize, b[1] as usize);
            black4[u] += 1; black4[v] += 1;
            black7[u].push(v); black7[v].push(u);
        }

        let mut black5 = Vec::with_capacity(black3);
        let mut black6 = black4.iter().position(|&d| d == 1).unwrap_or(0);
        let (mut curr, mut prev) = (black6, usize::MAX);
        while black5.len() < black3 {
            black5.push(curr);
            let next = black7[curr].iter().find(|&&x| x != prev);
            if let Some(&n) = next { prev = curr; curr = n; } else { break; }
        }

        let mut black8 = vec![0i64; black3];
        let mut black_vals: Vec<i64> = (1..=black1 as i64).collect();
        
        let mut b_dq = std::collections::VecDeque::new();
        let mut b_idx = black3 - 1;
        
        let mut left = true;
        while b_dq.len() < black3 {
            if left { b_dq.push_front(black_vals[b_idx]); } 
            else { b_dq.push_back(black_vals[b_idx]); }
            if b_idx > 0 { b_idx -= 1; }
            left = !left;
        }

        for (i, val) in b_dq.into_iter().enumerate() {
            black8[black5[i]] = val;
        }

        black2.iter().map(|b| black8[b[0] as usize] * black8[b[1] as usize]).sum()
    }
}
