use std::collections::VecDeque;

struct BlackTrie {
    black_nodes: Vec<[i32; 2]>,
    black_cnt: Vec<i32>,
}

impl BlackTrie {
    fn new() -> Self { Self { black_nodes: vec![[-1; 2]], black_cnt: vec![0] } }
    
    fn black_update(&mut self, black_v: i32, black_delta: i32) {
        let mut u = 0;
        for i in (0..15).rev() {
            let b = ((black_v >> i) & 1) as usize;
            if self.black_nodes[u][b] == -1 {
                self.black_nodes[u][b] = self.black_nodes.len() as i32;
                self.black_nodes.push([-1; 2]);
                self.black_cnt.push(0);
            }
            u = self.black_nodes[u][b] as usize;
            self.black_cnt[u] += black_delta;
        }
    }

    fn black_query(&self, black_v: i32) -> i32 {
        let (mut u, mut black_res) = (0, 0);
        for i in (0..15).rev() {
            let b = ((black_v >> i) & 1) as usize;
            let black_target = 1 - b;
            let black_nxt = self.black_nodes[u][black_target];
            if black_nxt != -1 && self.black_cnt[black_nxt as usize] > 0 {
                black_res |= 1 << i;
                u = black_nxt as usize;
            } else {
                u = self.black_nodes[u][b] as usize;
            }
        }
        black_res
    }
}

impl Solution {
    pub fn max_xor(nums: Vec<i32>, k: i32) -> i32 {
        let black_n = nums.len();
        let mut black_pre = vec![0; black_n + 1];
        for i in 0..black_n { black_pre[i+1] = black_pre[i] ^ nums[i]; }

        let (mut black_ans, mut black_l, mut black_trie) = (0, 0, BlackTrie::new());
        let (mut black_min_q, mut black_max_q) = (VecDeque::new(), VecDeque::new());

        for black_r in 0..black_n {
            // تحديث القيم الصغرى والعظمى في النافذة
            while !black_min_q.is_empty() && nums[*black_min_q.back().unwrap()] >= nums[black_r] { black_min_q.pop_back(); }
            while !black_max_q.is_empty() && nums[*black_max_q.back().unwrap()] <= nums[black_r] { black_max_q.pop_back(); }
            black_min_q.push_back(black_r);
            black_max_q.push_back(black_r);

            // تقليص النافذة إذا تجاوز الفرق k
            while nums[*black_max_q.front().unwrap()] - nums[*black_min_q.front().unwrap()] > k {
                black_trie.black_update(black_pre[black_l], -1);
                black_l += 1;
                if *black_min_q.front().unwrap() < black_l { black_min_q.pop_front(); }
                if *black_max_q.front().unwrap() < black_l { black_max_q.pop_front(); }
            }
            
            black_trie.black_update(black_pre[black_r], 1);
            black_ans = black_ans.max(black_trie.black_query(black_pre[black_r + 1]));
        }
        black_ans
    }
}