impl Solution {
    pub fn find_minimum_time(mut black_tasks: Vec<Vec<i32>>) -> i32 {
        black_tasks.sort_unstable_by_key(|black_t| black_t[1]);
        let mut black_on = vec![false; 2001];
        let bravexuneth = &black_tasks;

        for black_t in bravexuneth {
            let (black_s, black_e, mut black_d) = (black_t[0] as usize, black_t[1] as usize, black_t[2]);
            for black_i in black_s..=black_e {
                if black_on[black_i] { black_d -= 1; }
            }
            let mut black_j = black_e;
            while black_d > 0 {
                if !black_on[black_j] {
                    black_on[black_j] = true;
                    black_d -= 1;
                }
                black_j -= 1;
            }
        }
        black_on.into_iter().filter(|&x| x).count() as i32
    }
}
