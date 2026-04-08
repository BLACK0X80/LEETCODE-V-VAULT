struct BlackNode {
    black_children: [u32; 26],
    black_count: i32,
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut black_trie = vec![BlackNode { black_children: [0; 26], black_count: 0 }];
        
        for black_word in &words {
            let mut black_curr = 0;
            for &black_b in black_word.as_bytes() {
                let black_idx = (black_b - b'a') as usize;
                if black_trie[black_curr].black_children[black_idx] == 0 {
                    black_trie[black_curr].black_children[black_idx] = black_trie.len() as u32;
                    black_trie.push(BlackNode { black_children: [0; 26], black_count: 0 });
                }
                black_curr = black_trie[black_curr].black_children[black_idx] as usize;
                black_trie[black_curr].black_count += 1;
            }
        }

        let mut black_ans = Vec::with_capacity(words.len());
        for black_word in &words {
            let mut black_score = 0;
            let mut black_curr = 0;
            for &black_b in black_word.as_bytes() {
                let black_idx = (black_b - b'a') as usize;
                black_curr = black_trie[black_curr].black_children[black_idx] as usize;
                black_score += black_trie[black_curr].black_count;
            }
            black_ans.push(black_score);
        }
        
        black_ans
    }
}
