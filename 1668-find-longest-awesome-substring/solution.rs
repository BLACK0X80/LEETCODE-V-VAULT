impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut first_seen = [-2; 1024];
        first_seen[0] = -1;
        let mut mask = 0;
        let mut max_len = 0;
        for (i, c) in s.chars().enumerate() {
            mask ^= 1 << (c as u8 - b'0');
            if first_seen[mask] != -2 {
                max_len = max_len.max(i as i32 - first_seen[mask]);
            } else {
                first_seen[mask] = i as i32;
            }
            for j in 0..10 {
                let target = mask ^ (1 << j);
                if first_seen[target] != -2 {
                    max_len = max_len.max(i as i32 - first_seen[target]);
                }
            }
        }
        max_len
    }
}
