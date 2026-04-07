use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn most_booked(black1: i32, mut black2: Vec<Vec<i32>>) -> i32 {
        black2.sort_unstable_by_key(|black3| black3[0]);
        let mut black4 = BinaryHeap::new();
        let mut black5 = BinaryHeap::new();
        let mut black6 = vec![0; black1 as usize];
        for black7 in 0..black1 { black5.push(Reverse(black7)); }
        for black8 in black2 {
            let (black9, black10) = (black8[0] as i64, (black8[1] - black8[0]) as i64);
            while let Some(Reverse((black11, black12))) = black4.peek() {
                if *black11 <= black9 {
                    black5.push(Reverse(*black12));
                    black4.pop();
                } else { break; }
            }
            if let Some(Reverse(black13)) = black5.pop() {
                black6[black13 as usize] += 1;
                black4.push(Reverse((black9 + black10, black13)));
            } else {
                let Reverse((black14, black15)) = black4.pop().unwrap();
                black6[black15 as usize] += 1;
                black4.push(Reverse((black14 + black10, black15)));
            }
        }
        black6.iter().enumerate().max_by(|&(black16, &black17), &(black18, &black19)| 
            black17.cmp(&black19).then(black18.cmp(&black16))).unwrap().0 as i32
    }
}
