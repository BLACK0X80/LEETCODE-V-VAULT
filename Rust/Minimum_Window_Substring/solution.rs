impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let mut need = [0i32; 128];
        let mut missing = t.len() as i32;

        for c in t.bytes() { need[c as usize] += 1; }

        let (mut best_start, mut best_len) = (0, usize::MAX);
        let mut left = 0;

        for right in 0..s.len() {
            let c = s[right] as usize;
            if need[c] > 0 { missing -= 1; }
            need[c] -= 1;

            while missing == 0 {
                if right - left + 1 < best_len {
                    best_len = right - left + 1;
                    best_start = left;
                }
                let lc = s[left] as usize;
                need[lc] += 1;
                if need[lc] > 0 { missing += 1; }
                left += 1;
            }
        }

        if best_len == usize::MAX { String::new() }
        else { String::from_utf8(s[best_start..best_start + best_len].to_vec()).unwrap() }
    }
}