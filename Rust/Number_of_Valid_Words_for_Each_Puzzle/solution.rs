use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut black_freq: HashMap<u32, i32> = HashMap::new();
        for w in &words {
            let black_mask = w.bytes().fold(0u32, |m, b| m | (1 << (b - b'a')));
            *black_freq.entry(black_mask).or_insert(0) += 1;
        }
        puzzles.iter().map(|p| {
            let black_pmask = p.bytes().fold(0u32, |m, b| m | (1 << (b - b'a')));
            let black_first = 1u32 << (p.as_bytes()[0] - b'a');
            let mut black_sub = black_pmask;
            let mut black_ans = 0i32;
            loop {
                if black_sub & black_first != 0 {
                    black_ans += black_freq.get(&black_sub).unwrap_or(&0);
                }
                if black_sub == 0 { break; }
                black_sub = (black_sub - 1) & black_pmask;
            }
            black_ans
        }).collect()
    }
}