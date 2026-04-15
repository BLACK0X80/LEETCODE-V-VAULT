impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;

        words.sort_by_key(|w| w.len());
        let mut dict: HashSet<String> = HashSet::new();
        let mut result = vec![];

        for word in words {
            let n = word.len();
            if n == 0 { dict.insert(word); continue; }
            let w = word.as_bytes();
            let mut dp = vec![false; n + 1];
            dp[0] = true;
            for i in 1..=n {
                for j in (if i == n { 1 } else { 0 })..i {
                    if dp[j] && dict.contains(std::str::from_utf8(&w[j..i]).unwrap()) {
                        dp[i] = true;
                        break;
                    }
                }
            }
            if dp[n] { result.push(word.clone()); }
            dict.insert(word);
        }

        result
    }
}