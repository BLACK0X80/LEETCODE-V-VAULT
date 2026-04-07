use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn busiest_servers(black1: i32, black2: Vec<i32>, black3: Vec<i32>) -> Vec<i32> {
        let mut black4 = vec![0; black1 as usize];
        let mut black5: BTreeSet<i32> = (0..black1).collect();
        let mut black6 = BinaryHeap::new();
        for black7 in 0..black2.len() {
            let black8 = black2[black7];
            while let Some(Reverse((black9, black10))) = black6.peek() {
                if *black9 <= black8 {
                    black5.insert(*black10);
                    black6.pop();
                } else { break; }
            }
            if black5.is_empty() { continue; }
            let mut black11 = black5.range((black7 as i32 % black1)..).next();
            if black11.is_none() { black11 = black5.iter().next(); }
            let &black12 = black11.unwrap();
            black4[black12 as usize] += 1;
            black6.push(Reverse((black8 + black3[black7], black12)));
            black5.remove(&black12);
        }
        let black13 = *black4.iter().max().unwrap();
        black4.iter().enumerate().filter(|&(_, &black14)| black14 == black13).map(|(black15, _)| black15 as i32).collect()
    }
}
