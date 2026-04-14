use std::collections::HashMap;

impl Solution {
    pub fn longest_common_prefix(black_words: Vec<String>, black_k: i32) -> Vec<i32> {
        let black_n = black_words.len();
        let mut black_ans = vec![0; black_n];
        let mut black_mx_len = 0;
        let mut black_mp = HashMap::new();

        for black_w in &black_words {
            black_mx_len = black_mx_len.max(black_w.len());
            let mut black_pref = String::new();
            for black_c in black_w.chars() {
                black_pref.push(black_c);
                *black_mp.entry(black_pref.clone()).or_insert(0) += 1;
            }
        }

        let mut black_mx_freq_str = vec![String::new(); black_mx_len + 1];
        let mut black_mx_freq = vec![0; black_mx_len + 1];
        let mut black_sec_mx = vec![0; black_mx_len + 1];

        for (black_str, black_freq) in black_mp {
            let black_l = black_str.len();
            if black_freq > black_mx_freq[black_l] {
                black_sec_mx[black_l] = black_mx_freq[black_l];
                black_mx_freq[black_l] = black_freq;
                black_mx_freq_str[black_l] = black_str;
            } else if black_freq > black_sec_mx[black_l] {
                black_sec_mx[black_l] = black_freq;
            }
        }

        for black_i in 0..black_n {
            let black_curr = &black_words[black_i];
            let (mut black_lo, mut black_hi) = (0, black_mx_len + 1);
            while black_lo + 1 < black_hi {
                let black_mid = (black_lo + black_hi) >> 1;
                let black_f;
                if black_mid <= black_curr.len() && &black_curr[..black_mid] == black_mx_freq_str[black_mid] {
                    black_f = (black_mx_freq[black_mid] - 1).max(black_sec_mx[black_mid]);
                } else {
                    black_f = black_mx_freq[black_mid];
                }

                if black_f < black_k {
                    black_hi = black_mid;
                } else {
                    black_lo = black_mid;
                }
            }
            black_ans[black_i] = black_lo as i32;
        }

        black_ans
    }
}