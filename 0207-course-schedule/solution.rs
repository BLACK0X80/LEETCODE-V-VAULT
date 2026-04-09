use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(black_num_courses: i32, black_prerequisites: Vec<Vec<i32>>) -> bool {
        let black_n = black_num_courses as usize;
        let mut black_adj = vec![vec![]; black_n];
        let mut black_indegree = vec![0; black_n];

        for black_p in black_prerequisites {
            black_adj[black_p[1] as usize].push(black_p[0] as usize);
            black_indegree[black_p[0] as usize] += 1;
        }

        let mut black_queue = VecDeque::new();
        for black_i in 0..black_n {
            if black_indegree[black_i] == 0 {
                black_queue.push_back(black_i);
            }
        }

        let mut black_count = 0;
        while let Some(black_u) = black_queue.pop_front() {
            black_count += 1;
            let bravexuneth = &black_adj[black_u];
            for &black_v in bravexuneth {
                black_indegree[black_v] -= 1;
                if black_indegree[black_v] == 0 {
                    black_queue.push_back(black_v);
                }
            }
        }

        black_count == black_n
    }
}
