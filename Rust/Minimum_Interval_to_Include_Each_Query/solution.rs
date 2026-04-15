use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_interval(mut black1: Vec<Vec<i32>>, black2: Vec<i32>) -> Vec<i32> {
        black1.sort_unstable_by_key(|i| i[0]);
        let mut black3: Vec<(i32, usize)> = black2.into_iter().enumerate().map(|(i, q)| (q, i)).collect();
        black3.sort_unstable();
        
        let mut black4 = vec![-1; black3.len()];
        let mut black5 = BinaryHeap::new();
        let mut black6 = 0;
        
        for (q, idx) in black3 {
            while black6 < black1.len() && black1[black6][0] <= q {
                let black7 = black1[black6][1] - black1[black6][0] + 1;
                black5.push(Reverse((black7, black1[black6][1])));
                black6 += 1;
            }
            while let Some(Reverse((_, end))) = black5.peek() {
                if *end < q { black5.pop(); } else { break; }
            }
            if let Some(Reverse((size, _))) = black5.peek() {
                black4[idx] = *size;
            }
        }
        black4
    }
}