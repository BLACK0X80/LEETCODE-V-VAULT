impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut k = 0;
        let mut s = word.clone();
        while sequence.contains(&s) {
            k += 1;
            s.push_str(&word);
        }
        k
    }
}