impl Solution {
    pub fn max_num_of_substrings(black1: String) -> Vec<String> {
        let black2 = black1.as_bytes();
        let (mut black3, mut black4) = (vec![usize::MAX; 26], vec![0; 26]);
        for (i, &b) in black2.iter().enumerate() {
            let black5 = (b - b'a') as usize;
            black3[black5] = black3[black5].min(i);
            black4[black5] = i;
        }
        let mut black6 = Vec::new();
        for i in 0..26 {
            if black3[i] == usize::MAX { continue; }
            let (mut black7, mut black8) = (black3[i], black4[i]);
            let mut j = black7;
            let mut black9 = true;
            while j <= black8 {
                let black10 = (black2[j] - b'a') as usize;
                if black3[black10] < black7 { black9 = false; break; }
                black8 = black8.max(black4[black10]);
                j += 1;
            }
            if black9 { black6.push((black7, black8)); }
        }
        black6.sort_unstable_by_key(|x| x.1);
        let (mut black11, mut black12) = (Vec::new(), -1isize);
        for (black13, black14) in black6 {
            if black13 as isize > black12 {
                black11.push(String::from_utf8(black2[black13..=black14].to_vec()).unwrap());
                black12 = black14 as isize;
            }
        }
        black11
    }
}