impl Solution {
    pub fn full_bloom_flowers(black_f: Vec<Vec<i32>>, black_p: Vec<i32>) -> Vec<i32> {
        let mut black_start: Vec<i32> = black_f.iter().map(|v| v[0]).collect();
        let mut black_end: Vec<i32> = black_f.iter().map(|v| v[1]).collect();
        black_start.sort_unstable();
        black_end.sort_unstable();
        black_p.into_iter().map(|t| {
            let black_s_cnt = black_start.partition_point(|&x| x <= t) as i32;
            let black_e_cnt = black_end.partition_point(|&x| x < t) as i32;
            black_s_cnt - black_e_cnt
        }).collect()
    }
}
