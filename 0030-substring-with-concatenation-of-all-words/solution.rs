use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let word_len = words[0].len();
        let word_count = words.len();
        let mut result = Vec::new();

        let mut word_freq: HashMap<&[u8], i32> = HashMap::new();
        for word in &words {
            *word_freq.entry(word.as_bytes()).or_insert(0) += 1;
        }

        for start in 0..word_len {
            let mut left = start;
            let mut right = start;
            let mut count = 0;
            let mut window: HashMap<&[u8], i32> = HashMap::new();

            while right + word_len <= n {
                let word = &s[right..right + word_len];
                right += word_len;

                if let Some(&freq) = word_freq.get(word) {
                    *window.entry(word).or_insert(0) += 1;
                    count += 1;

                    while window[word] > freq {
                        let left_word = &s[left..left + word_len];
                        *window.get_mut(left_word).unwrap() -= 1;
                        count -= 1;
                        left += word_len;
                    }

                    if count == word_count {
                        result.push(left as i32);
                    }
                } else {
                    window.clear();
                    count = 0;
                    left = right;
                }
            }
        }

        result
    }
}
