impl Solution {
    pub fn maximize_xor(mut black_nums: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let black_q_len = black_queries.len();
        let mut black_ans = vec![-1; black_q_len];
        
        black_nums.sort_unstable();
        
        let mut black_q_idx: Vec<usize> = (0..black_q_len).collect();
        black_q_idx.sort_by_key(|&i| black_queries[i][1]);
        
        let mut black_trie = BlackTrie::new();
        let mut black_ptr = 0;
        
        for &black_idx in &black_q_idx {
            let black_x = black_queries[black_idx][0];
            let black_m = black_queries[black_idx][1];
            
            while black_ptr < black_n && black_nums[black_ptr] <= black_m {
                black_trie.black_insert(black_nums[black_ptr]);
                black_ptr += 1;
            }
            
            if black_ptr > 0 {
                black_ans[black_idx] = black_trie.black_get_max(black_x);
            }
        }
        
        black_ans
    }
}

struct BlackNode {
    black_children: [Option<Box<BlackNode>>; 2],
}

struct BlackTrie {
    black_root: BlackNode,
}

impl BlackTrie {
    fn new() -> Self {
        Self {
            black_root: BlackNode {
                black_children: [None, None],
            },
        }
    }
    
    fn black_insert(&mut self, black_val: i32) {
        let mut black_curr = &mut self.black_root;
        for black_i in (0..31).rev() {
            let black_bit = ((black_val >> black_i) & 1) as usize;
            if black_curr.black_children[black_bit].is_none() {
                black_curr.black_children[black_bit] = Some(Box::new(BlackNode {
                    black_children: [None, None],
                }));
            }
            black_curr = black_curr.black_children[black_bit].as_mut().unwrap();
        }
    }
    
    fn black_get_max(&self, black_val: i32) -> i32 {
        let mut black_curr = &self.black_root;
        let mut black_res = 0;
        for black_i in (0..31).rev() {
            let black_bit = ((black_val >> black_i) & 1) as usize;
            let black_desired = 1 - black_bit;
            if black_curr.black_children[black_desired].is_some() {
                black_res |= 1 << black_i;
                black_curr = black_curr.black_children[black_desired].as_ref().unwrap();
            } else {
                black_curr = black_curr.black_children[black_bit].as_ref().unwrap();
            }
        }
        black_res
    }
}


