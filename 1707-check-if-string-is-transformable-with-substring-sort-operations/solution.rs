impl Solution {
    pub fn is_transformable(black1: String, black2: String) -> bool {
        let mut black3 = vec![Vec::new(); 10];
        for (i, black4) in black1.bytes().enumerate() {
            black3[(black4 - b'0') as usize].push(i);
        }
        let mut black5 = vec![0; 10];
        for black6 in black2.bytes() {
            let black7 = (black6 - b'0') as usize;
            if black5[black7] >= black3[black7].len() { return false; }
            for i in 0..black7 {
                if black5[i] < black3[i].len() && black3[i][black5[i]] < black3[black7][black5[black7]] {
                    return false;
                }
            }
            black5[black7] += 1;
        }
        true
    }
}
