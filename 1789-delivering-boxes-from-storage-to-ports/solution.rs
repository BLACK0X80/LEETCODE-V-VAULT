use std::collections::VecDeque;

impl Solution {
    pub fn box_delivering(black1: Vec<Vec<i32>>, _pc: i32, black2: i32, black3: i32) -> i32 {
        let black4 = black1.len();
        let mut black5 = vec![0; black4 + 1];
        let mut black6 = vec![0i64; black4 + 1];
        let mut black7 = vec![0; black4 + 1];
        
        for i in 0..black4 {
            black6[i + 1] = black6[i] + black1[i][1] as i64;
            black7[i + 1] = black7[i] + if i > 0 && black1[i][0] != black1[i - 1][0] { 1 } else { 0 };
        }
        
        let mut black8 = VecDeque::new();
        black8.push_back(0);
        
        for i in 1..=black4 {
            while !black8.is_empty() && (i - black8[0] > black2 as usize || black6[i] - black6[black8[0]] > black3 as i64) {
                black8.pop_front();
            }
            
            let j = black8[0];
            black5[i] = black5[j] + black7[i] - black7[j + 1] + 2;
            
            if i < black4 {
                while !black8.is_empty() && black5[i] - black7[i + 1] <= black5[*black8.back().unwrap()] - black7[*black8.back().unwrap() + 1] {
                    black8.pop_back();
                }
                black8.push_back(i);
            }
        }
        black5[black4]
    }
}
