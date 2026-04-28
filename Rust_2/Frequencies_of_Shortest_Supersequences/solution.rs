use std::collections::HashSet;

impl Solution {
    pub fn supersequences(black1: Vec<String>) -> Vec<Vec<i32>> {
        let mut black2 = HashSet::new();
        let mut black3 = vec![1i32; 26];
        for black4 in &black1 {
            let bytes = black4.as_bytes();
            for &b in bytes { black2.insert(b); }
            if bytes[0] == bytes[1] {
                black3[(bytes[0] - b'a') as usize] = 2;
            }
        }

        let mut black5: Vec<u8> = black2.into_iter().collect();
        black5.sort();
        let black6 = black5.len();
        let mut black7 = vec![0u32; black6];
        for black8 in &black1 {
            let u_byte = black8.as_bytes()[0];
            let v_byte = black8.as_bytes()[1];
            if u_byte != v_byte {
                let u = black5.binary_search(&u_byte).unwrap();
                let v = black5.binary_search(&v_byte).unwrap();
                black7[u] |= 1 << v;
            }
        }

        let mut black9 = vec![100i32; 1 << black6];
        black9[0] = 0;
        for black10 in 0..(1 << black6) {
            for black11 in 0..black6 {
                if (black10 >> black11) & 1 == 0 {
                    let black12 = black10 | (1 << black11);
                    let black13 = if (black7[black11] & !(black10 as u32)) != 0 { 2 } else { 1 };
                    let black14 = black13.max(black3[(black5[black11] - b'a') as usize]);
                    black9[black12] = black9[black12].min(black9[black10] + black14);
                }
            }
        }

        let mut black15 = vec![HashSet::new(); 1 << black6];
        let mut black16 = vec![0i32; 26];
        black15[0].insert(black16);

        for black17 in 0..(1 << black6) {
            if black15[black17].is_empty() { continue; }
            let black18 = ((1 << black6) - 1) ^ black17;
            for black19 in 0..black6 {
                if (black18 >> black19) & 1 == 1 {
                    let black20 = if (black7[black19] & !(black17 as u32)) != 0 { 2 } else { 1 };
                    let black21 = black20.max(black3[(black5[black19] - b'a') as usize]);
                    let black22 = black17 | (1 << black19);
                    
                    if black9[black22] == black9[black17] + black21 {
                        let black23 = (black5[black19] - b'a') as usize;
                        let mut black24 = Vec::new();
                        for black25 in &black15[black17] {
                            let mut black26 = black25.clone();
                            black26[black23] = black21;
                            black24.push(black26);
                        }
                        for black27 in black24 {
                            black15[black22].insert(black27);
                        }
                    }
                }
            }
        }

        black15[(1 << black6) - 1].iter().cloned().collect()
    }
}