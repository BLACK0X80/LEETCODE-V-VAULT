use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct BlackNode {
    black_cost: i64,
    black_l: usize,
    black_r: usize,
    black_dead: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct BlackItem {
    black_cost: i64,
    black_idx: usize,
}

impl Ord for BlackItem {
    fn cmp(&self, black_other: &Self) -> Ordering {
        black_other.black_cost.cmp(&self.black_cost)
            .then_with(|| black_other.black_idx.cmp(&self.black_idx))
    }
}

impl PartialOrd for BlackItem {
    fn partial_cmp(&self, black_other: &Self) -> Option<Ordering> {
        Some(self.cmp(black_other))
    }
}

impl Solution {
    pub fn min_operations(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let black_n = black_nums.len();
        let mut black_k = black_k as usize;
        if black_k > black_n / 2 { return -1; }
        if black_k == 0 { return 0; }

        let mut black_nodes = vec![BlackNode { black_cost: 0, black_l: 0, black_r: 0, black_dead: false }; black_n];
        let mut black_pq = BinaryHeap::new();

        for black_i in 0..black_n {
            let black_left_idx = if black_i == 0 { black_n - 1 } else { black_i - 1 };
            let black_right_idx = if black_i == black_n - 1 { 0 } else { black_i + 1 };
            
            let black_target = black_nums[black_left_idx].max(black_nums[black_right_idx]) as i64 + 1;
            let black_cost = 0.max(black_target - black_nums[black_i] as i64);
            
            black_nodes[black_i] = BlackNode {
                black_cost,
                black_l: black_left_idx,
                black_r: black_right_idx,
                black_dead: false,
            };
            black_pq.push(BlackItem { black_cost, black_idx: black_i });
        }

        let mut black_ans = 0i64;

        while black_k > 0 && !black_pq.is_empty() {
            let BlackItem { black_cost, black_idx: black_u } = black_pq.pop().unwrap();
            
            if black_nodes[black_u].black_dead { continue; }

            black_ans += black_cost;
            black_k -= 1;

            let black_l = black_nodes[black_u].black_l;
            let black_r = black_nodes[black_u].black_r;

            black_nodes[black_l].black_dead = true;
            black_nodes[black_r].black_dead = true;

            black_nodes[black_u].black_cost = black_nodes[black_l].black_cost + black_nodes[black_r].black_cost - black_nodes[black_u].black_cost;
            black_pq.push(BlackItem { black_cost: black_nodes[black_u].black_cost, black_idx: black_u });

            black_nodes[black_u].black_l = black_nodes[black_l].black_l;
            black_nodes[black_u].black_r = black_nodes[black_r].black_r;
            
            let black_new_l = black_nodes[black_u].black_l;
            let black_new_r = black_nodes[black_u].black_r;
            
            black_nodes[black_new_l].black_r = black_u;
            black_nodes[black_new_r].black_l = black_u;
        }

        black_ans as i32
    }
}