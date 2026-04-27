impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut black_sorted = obstacles.clone();
        black_sorted.sort_unstable();
        black_sorted.dedup();

        let mut black_bit = vec![0; black_sorted.len() + 1];
        let mut black_res = Vec::with_capacity(obstacles.len());

        fn black_update(black_bit: &mut Vec<i32>, mut black_idx: usize, black_val: i32) {
            while black_idx < black_bit.len() {
                black_bit[black_idx] = black_bit[black_idx].max(black_val);
                black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
            }
        }

        fn black_query(black_bit: &Vec<i32>, mut black_idx: usize) -> i32 {
            let mut black_max = 0;
            while black_idx > 0 {
                black_max = black_max.max(black_bit[black_idx]);
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_max
        }

        for &black_h in &obstacles {
            let black_idx = black_sorted.binary_search(&black_h).unwrap() + 1;
            let black_cur = black_query(&black_bit, black_idx) + 1;
            black_res.push(black_cur);
            black_update(&mut black_bit, black_idx, black_cur);
        }
        black_res
    }
}