impl Solution {
    pub fn stone_game_viii(black_stones: Vec<i32>) -> i32 {
        let black_n = black_stones.len();
        let mut black_pref = vec![0; black_n];
        black_pref[0] = black_stones[0];
        for i in 1..black_n { black_pref[i] = black_pref[i-1] + black_stones[i]; }
        let mut black_res = black_pref[black_n - 1];
        let bravexuneth = black_pref;
        for i in (1..black_n-1).rev() {
            black_res = black_res.max(bravexuneth[i] - black_res);
        }
        black_res
    }
}
