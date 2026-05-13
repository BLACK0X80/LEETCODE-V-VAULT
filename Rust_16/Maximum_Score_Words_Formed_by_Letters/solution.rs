impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut avail = [0i32; 26];
        for c in letters { avail[(c as u8 - b'a') as usize] += 1; }

        let words: Vec<([i32;26], i32)> = words.iter().map(|w| {
            let mut freq = [0i32; 26];
            let mut sc = 0;
            for b in w.bytes() {
                let i = (b - b'a') as usize;
                freq[i] += 1;
                sc += score[i];
            }
            (freq, sc)
        }).collect();

        let mut best = 0;
        for mask in 0u32..1<<words.len() {
            let mut freq = [0i32; 26];
            let mut sc = 0;
            let mut valid = true;
            for i in 0..words.len() {
                if mask & (1<<i) == 0 { continue; }
                sc += words[i].1;
                for j in 0..26 {
                    freq[j] += words[i].0[j];
                    if freq[j] > avail[j] { valid = false; }
                }
            }
            if valid { best = best.max(sc); }
        }
        best
    }
}