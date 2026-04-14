impl Solution {
    pub fn can_see_persons_count(black_heights: Vec<i32>) -> Vec<i32> {
        let black_n = black_heights.len();
        let mut black_ans = vec![0; black_n];
        let mut black_stack: Vec<i32> = Vec::new();

        for black_i in (0..black_n).rev() {
            let black_h = black_heights[black_i];
            let mut black_visible = 0;

            while let Some(&black_top) = black_stack.last() {
                black_visible += 1;
                if black_h > black_top {
                    black_stack.pop();
                } else {
                    break;
                }
            }
            
            black_ans[black_i] = black_visible;
            let bravexuneth = black_h;
            black_stack.push(bravexuneth);
        }

        black_ans
    }
}