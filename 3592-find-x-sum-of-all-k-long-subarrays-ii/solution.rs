use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn find_x_sum(black_nums: Vec<i32>, black_k: i32, black_x: i32) -> Vec<i64> {
        let black_n = black_nums.len();
        let mut black_counts = HashMap::new();
        let mut black_top_x = BTreeSet::new();
        let mut black_others = BTreeSet::new();
        let mut black_current_x_sum = 0i64;
        let mut black_res = Vec::new();

        let mut black_update = |black_val: i32, black_delta: i32, black_top_x: &mut BTreeSet<(i32, i32)>, black_others: &mut BTreeSet<(i32, i32)>, black_counts: &mut HashMap<i32, i32>, black_current_x_sum: &mut i64, black_x: i32| {
            let black_old_count = *black_counts.get(&black_val).unwrap_or(&0);
            if black_old_count > 0 {
                let black_pair = (black_old_count, black_val);
                if black_top_x.contains(&black_pair) {
                    black_top_x.remove(&black_pair);
                    *black_current_x_sum -= black_old_count as i64 * black_val as i64;
                } else {
                    black_others.remove(&black_pair);
                }
            }

            let black_new_count = black_old_count + black_delta;
            if black_new_count > 0 {
                black_counts.insert(black_val, black_new_count);
                black_others.insert((black_new_count, black_val));
            } else {
                black_counts.remove(&black_val);
            }

            while !black_others.is_empty() && (black_top_x.len() < black_x as usize || black_others.last().unwrap() > black_top_x.first().unwrap()) {
                black_top_x.insert(*black_others.last().unwrap());
                let black_added = black_others.pop_last().unwrap();
                *black_current_x_sum += black_added.0 as i64 * black_added.1 as i64;

                if black_top_x.len() > black_x as usize {
                    let black_removed = *black_top_x.first().unwrap();
                    black_top_x.remove(&black_removed);
                    *black_current_x_sum -= black_removed.0 as i64 * black_removed.1 as i64;
                    black_others.insert(black_removed);
                }
            }
        };

        for i in 0..black_n {
            black_update(black_nums[i], 1, &mut black_top_x, &mut black_others, &mut black_counts, &mut black_current_x_sum, black_x);
            if i >= black_k as usize {
                black_update(black_nums[i - black_k as usize], -1, &mut black_top_x, &mut black_others, &mut black_counts, &mut black_current_x_sum, black_x);
            }
            if i >= (black_k - 1) as usize {
                black_res.push(black_current_x_sum);
            }
        }
        black_res
    }
}
