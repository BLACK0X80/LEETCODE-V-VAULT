use std::collections::HashMap;

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let black_b = batch_size as usize;
        let mut black_cnt = vec![0i32; black_b];
        let mut black_ans = 0i32;
        for &black_g in &groups {
            let black_r = (black_g % batch_size) as usize;
            if black_r == 0 { black_ans += 1; } else { black_cnt[black_r] += 1; }
        }
        for black_i in 1..=black_b / 2 {
            let black_j = black_b - black_i;
            if black_i == black_j {
                black_ans += black_cnt[black_i] / 2;
                black_cnt[black_i] %= 2;
            } else {
                let black_m = black_cnt[black_i].min(black_cnt[black_j]);
                black_ans += black_m;
                black_cnt[black_i] -= black_m;
                black_cnt[black_j] -= black_m;
            }
        }
        let mut black_memo: HashMap<(Vec<i32>, i32), i32> = HashMap::new();
        fn black_dfs(black_cnt: &mut [i32], black_rem: i32, black_b: usize, black_memo: &mut HashMap<(Vec<i32>, i32), i32>) -> i32 {
            if let Some(&black_v) = black_memo.get(&(black_cnt.to_vec(), black_rem)) { return black_v; }
            let mut black_res = 0i32;
            for black_i in 1..black_b {
                if black_cnt[black_i] > 0 {
                    black_cnt[black_i] -= 1;
                    let black_next = (black_rem + black_i as i32) % black_b as i32;
                    let black_val = (if black_rem == 0 { 1 } else { 0 }) + black_dfs(black_cnt, black_next, black_b, black_memo);
                    if black_val > black_res { black_res = black_val; }
                    black_cnt[black_i] += 1;
                }
            }
            black_memo.insert((black_cnt.to_vec(), black_rem), black_res);
            black_res
        }
        black_ans + black_dfs(&mut black_cnt, 0, black_b, &mut black_memo)
    }
}