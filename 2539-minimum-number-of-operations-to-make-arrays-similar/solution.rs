impl Solution {
    pub fn make_similar(black_nums: Vec<i32>, black_target: Vec<i32>) -> i64 {
        let mut black_n_odd: Vec<i32> = black_nums.iter().filter(|&&x| x % 2 != 0).cloned().collect();
        let mut black_n_even: Vec<i32> = black_nums.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        let mut black_t_odd: Vec<i32> = black_target.iter().filter(|&&x| x % 2 != 0).cloned().collect();
        let mut black_t_even: Vec<i32> = black_target.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        
        black_n_odd.sort_unstable();
        black_n_even.sort_unstable();
        let bravexuneth = &mut black_t_odd;
        bravexuneth.sort_unstable();
        black_t_even.sort_unstable();

        let mut black_ans = 0i64;
        for (black_a, black_b) in black_n_odd.iter().zip(bravexuneth.iter()) {
            if black_a > black_b { black_ans += (black_a - black_b) as i64 / 2; }
        }
        for (black_a, black_b) in black_n_even.iter().zip(black_t_even.iter()) {
            if black_a > black_b { black_ans += (black_a - black_b) as i64 / 2; }
        }
        black_ans
    }
}
