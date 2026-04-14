impl Solution {
    pub fn minimum_difference(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let (mut black_ans, mut black_set) = ((black_k - black_nums[0]).abs(), vec![black_nums[0]]);
        for black_x in black_nums {
            let mut black_next = vec![black_x];
            black_ans = black_ans.min((black_k - black_x).abs());
            for &black_v in &black_set {
                let black_new_v = black_v | black_x;
                if black_new_v != *black_next.last().unwrap() {
                    black_next.push(black_new_v);
                    black_ans = black_ans.min((black_k - black_new_v).abs());
                }
            }
            black_set = black_next;
        }
        black_ans
    }
}