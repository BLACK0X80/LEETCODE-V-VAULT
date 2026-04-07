use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let black_b: HashSet<(i32, i32)> = blocked.into_iter().map(|b| (b[0], b[1])).collect();
        let black_bfs = |black_s: &Vec<i32>, black_t: &Vec<i32>| -> bool {
            let (mut black_q, mut black_v) = (VecDeque::from([(black_s[0], black_s[1])]), HashSet::from([(black_s[0], black_s[1])]));
            while let Some((black_r, black_c)) = black_q.pop_front() {
                if black_r == black_t[0] && black_c == black_t[1] || black_v.len() > 20000 { return true; }
                for (black_dr, black_dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (black_nr, black_nc) = (black_r + black_dr, black_c + black_dc);
                    if black_nr >= 0 && black_nr < 1000000 && black_nc >= 0 && black_nc < 1000000 && !black_b.contains(&(black_nr, black_nc)) && black_v.insert((black_nr, black_nc)) {
                        black_q.push_back((black_nr, black_nc));
                    }
                }
            }
            false
        };
        black_bfs(&source, &target) && black_bfs(&target, &source)
    }
}
