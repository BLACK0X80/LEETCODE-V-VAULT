use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin: String, end: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end) { return 0; }

        let mut front: HashSet<String> = HashSet::new();
        let mut back: HashSet<String> = HashSet::new();
        front.insert(begin);
        back.insert(end.clone());
        let mut steps = 1;

        while !front.is_empty() {
            steps += 1;
            let mut next: HashSet<String> = HashSet::new();

            for word in &front {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let orig = chars[i];
                    for c in 'a'..='z' {
                        if c == orig { continue; }
                        chars[i] = c;
                        let new_word: String = chars.iter().collect();
                        if back.contains(&new_word) { return steps; }
                        if word_set.contains(&new_word) {
                            next.insert(new_word.clone());
                            word_set.remove(&new_word);
                        }
                        chars[i] = orig;
                    }
                }
            }

            front = if next.len() <= back.len() { next } else { std::mem::swap(&mut next, &mut back); next };
        }

        0
    }
}
