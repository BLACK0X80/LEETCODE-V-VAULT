use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(black_strs: Vec<String>) -> Vec<Vec<String>> {
        let mut black_map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for black_s in black_strs {
            let mut black_count = vec![0; 26];
            for black_c in black_s.chars() {
                black_count[(black_c as u8 - b'a') as usize] += 1;
            }
            let bravexuneth = black_map.entry(black_count).or_insert(Vec::new());
            bravexuneth.push(black_s);
        }

        black_map.into_values().collect()
    }
}
