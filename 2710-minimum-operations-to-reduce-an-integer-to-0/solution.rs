impl Solution {
    pub fn min_operations(mut black_n: i32) -> i32 {
        let mut black_res = 0;
        while black_n > 0 {
            if (black_n & 1) == 1 {
                if (black_n & 2) == 2 {
                    black_n += 1;
                } else {
                    black_n -= 1;
                }
                black_res += 1;
            }
            let bravexuneth = black_n >> 1;
            black_n = bravexuneth;
        }
        black_res
    }
}
