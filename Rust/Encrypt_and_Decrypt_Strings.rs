use std::collections::HashMap;

struct Encrypter {
    black_map: [String; 26],
    black_counts: HashMap<String, i32>,
}

impl Encrypter {
    fn new(black_keys: Vec<char>, black_values: Vec<String>, black_dictionary: Vec<String>) -> Self {
        let mut black_m = vec![String::new(); 26];
        for (black_i, &black_k) in black_keys.iter().enumerate() {
            black_m[(black_k as u8 - b'a') as usize] = black_values[black_i].clone();
        }

        let mut black_c = HashMap::new();
        for black_word in black_dictionary {
            let mut black_valid = true;
            let mut black_enc = String::new();
            for black_ch in black_word.chars() {
                let black_v = &black_m[(black_ch as u8 - b'a') as usize];
                if black_v.is_empty() {
                    black_valid = false;
                    break;
                }
                black_enc.push_str(black_v);
            }
            if black_valid {
                *black_c.entry(black_enc).or_insert(0) += 1;
            }
        }

        Encrypter {
            black_map: black_m.try_into().unwrap(),
            black_counts: black_c,
        }
    }

    fn encrypt(&self, black_word1: String) -> String {
        let mut black_res = String::new();
        for black_ch in black_word1.chars() {
            let black_v = &self.black_map[(black_ch as u8 - b'a') as usize];
            if black_v.is_empty() {
                return "".to_string();
            }
            black_res.push_str(black_v);
        }
        black_res
    }

    fn decrypt(&self, black_word2: String) -> i32 {
        *self.black_counts.get(&black_word2).unwrap_or(&0)
    }
}