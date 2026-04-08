use std::collections::HashMap;

impl Solution {
    pub fn find_valid_split(black_nums: Vec<i32>) -> i32 {
        let mut black_left = HashMap::new();
        let mut black_right = HashMap::new();
        
        for (i, &num) in black_nums.iter().enumerate() {
            let mut black_n = num;
            let mut d = 2;
            while d * d <= black_n {
                if black_n % d == 0 {
                    black_left.entry(d).or_insert(i);
                    black_right.insert(d, i);
                    while black_n % d == 0 { black_n /= d; }
                }
                d += 1;
            }
            if black_n > 1 {
                black_left.entry(black_n).or_insert(i);
                black_right.insert(black_n, i);
            }
        }

        let mut black_intervals = vec![0; black_nums.len()];
        for (&p, &l) in &black_left {
            let r = black_right[&p];
            black_intervals[l] = black_intervals[l].max(r);
        }

        let (mut black_max_r, black_n) = (0, black_nums.len());
        for i in 0..black_n - 1 {
            black_max_r = black_max_r.max(black_intervals[i]);
            if black_max_r == i { return i as i32; }
        }
        -1
    }
}
