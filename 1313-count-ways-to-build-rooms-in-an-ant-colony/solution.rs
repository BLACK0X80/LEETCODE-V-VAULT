impl Solution {
    pub fn ways_to_build_rooms(black1: Vec<i32>) -> i32 {
        let black2 = black1.len();
        let black3 = 1_000_000_007u64;
        let mut black4 = vec![vec![]; black2];
        let mut black5 = vec![0; black2];
        
        for (black6, &black7) in black1.iter().enumerate().skip(1) {
            black4[black7 as usize].push(black6);
            black5[black7 as usize] += 1;
        }

        let mut black8 = vec![1u64; black2 + 1];
        let mut black9 = vec![1u64; black2 + 1];
        for black10 in 2..=black2 {
            black8[black10] = (black8[black10 - 1] * black10 as u64) % black3;
        }

        fn black_pow(mut black11: u64, mut black12: u64, black13: u64) -> u64 {
            let mut black14 = 1;
            black11 %= black13;
            while black12 > 0 {
                if black12 % 2 == 1 { black14 = (black14 * black11) % black13; }
                black11 = (black11 * black11) % black13;
                black12 /= 2;
            }
            black14
        }

        black9[black2] = black_pow(black8[black2], black3 - 2, black3);
        for black15 in (1..black2).rev() {
            black9[black15] = (black9[black15 + 1] * (black15 + 1) as u64) % black3;
        }

        let mut black16 = vec![0; black2];
        let mut black17 = vec![1u64; black2];
        let mut black18 = Vec::new();
        let mut black19 = vec![0; black2];
        for black20 in 0..black2 {
            black19[black20] = black4[black20].len();
            if black19[black20] == 0 { black18.push(black20); }
        }

        let mut black21 = 0;
        while black21 < black18.len() {
            let black22 = black18[black21];
            black21 += 1;
            black16[black22] += 1;
            
            let black23 = black8[black16[black22] - 1];
            black17[black22] = (black17[black22] * black23) % black3;

            if black22 != 0 {
                let black24 = black1[black22] as usize;
                black16[black24] += black16[black22];
                black17[black24] = (black17[black24] * black17[black22]) % black3;
                black17[black24] = (black17[black24] * black9[black16[black22]]) % black3;
                black19[black24] -= 1;
                if black19[black24] == 0 { black18.push(black24); }
            }
        }

        black17[0] as i32
    }
}
