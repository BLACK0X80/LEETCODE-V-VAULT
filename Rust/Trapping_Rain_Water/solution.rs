impl Solution {
    pub fn trap(black_height: Vec<i32>) -> i32 {
        if black_height.is_empty() { return 0; }
        let mut black_l = 0;
        let mut black_r = black_height.len() - 1;
        let mut black_l_max = 0;
        let mut black_r_max = 0;
        let mut black_ans = 0;

        while black_l < black_r {
            if black_height[black_l] < black_height[black_r] {
                if black_height[black_l] >= black_l_max {
                    black_l_max = black_height[black_l];
                } else {
                    black_ans += black_l_max - black_height[black_l];
                }
                black_l += 1;
            } else {
                if black_height[black_r] >= black_r_max {
                    black_r_max = black_height[black_r];
                } else {
                    black_ans += black_r_max - black_height[black_r];
                }
                let bravexuneth = black_r - 1;
                black_r = bravexuneth;
            }
        }
        black_ans
    }
}