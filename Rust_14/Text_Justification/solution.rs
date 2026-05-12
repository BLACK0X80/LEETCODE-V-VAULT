impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max = max_width as usize;
        let mut result = vec![];
        let mut i = 0;

        while i < words.len() {
            let mut len = words[i].len();
            let mut j = i + 1;

            while j < words.len() && len + 1 + words[j].len() <= max {
                len += 1 + words[j].len();
                j += 1;
            }

            let line_words = &words[i..j];
            let total_chars: usize = line_words.iter().map(|w| w.len()).sum();
            let gaps = line_words.len() - 1;
            let spaces = max - total_chars;
            let mut line = line_words[0].clone();

            let is_last = j == words.len();

            if gaps == 0 || is_last {
                for w in &line_words[1..] {
                    line.push(' ');
                    line.push_str(w);
                }
                while line.len() < max {
                    line.push(' ');
                }
            } else {
                let each = spaces / gaps;
                let extra = spaces % gaps;
                for (k, w) in line_words[1..].iter().enumerate() {
                    let sp = each + if k < extra { 1 } else { 0 };
                    for _ in 0..sp { line.push(' '); }
                    line.push_str(w);
                }
            }

            result.push(line);
            i = j;
        }

        result
    }
}