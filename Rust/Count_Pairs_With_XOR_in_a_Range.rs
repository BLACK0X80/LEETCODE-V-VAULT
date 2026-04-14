impl Solution {
    pub fn count_pairs(black_nums: Vec<i32>, black_low: i32, black_high: i32) -> i32 {
        let mut black_trie = vec![[0, 0, 0]];
        let mut black_ans = 0;
        
        let mut black_f = |black_limit: i32, black_val: i32, black_t: &Vec<[i32; 3]>| {
            let (mut black_curr, mut black_res) = (0, 0);
            for black_i in (0..15).rev() {
                let black_v_b = ((black_val >> black_i) & 1) as usize;
                let black_l_b = ((black_limit >> black_i) & 1) as usize;
                if black_l_b == 1 {
                    let black_left = black_t[black_curr][black_v_b] as usize;
                    if black_left != 0 { black_res += black_t[black_left][2]; }
                    black_curr = black_t[black_curr][1 - black_v_b] as usize;
                } else {
                    black_curr = black_t[black_curr][black_v_b] as usize;
                }
                if black_curr == 0 { return black_res; }
            }
            black_res + black_t[black_curr][2]
        };

        for &black_x in &black_nums {
            black_ans += black_f(black_high, black_x, &black_trie) - black_f(black_low - 1, black_x, &black_trie);
            let mut black_curr = 0;
            for black_i in (0..15).rev() {
                let black_b = ((black_x >> black_i) & 1) as usize;
                if black_trie[black_curr][black_b] == 0 {
                    black_trie[black_curr][black_b] = black_trie.len() as i32;
                    black_trie.push([0, 0, 0]);
                }
                black_curr = black_trie[black_curr][black_b] as usize;
                black_trie[black_curr][2] += 1;
            }
        }
        black_ans
    }
}