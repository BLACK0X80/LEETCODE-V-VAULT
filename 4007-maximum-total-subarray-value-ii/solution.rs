use std::collections::BinaryHeap;

struct BlackTable {
    black1: Vec<Vec<i32>>,
    black2: Vec<Vec<i32>>,
    black3: Vec<usize>,
}

impl BlackTable {
    fn new(black4: &[i32]) -> Self {
        let black5 = black4.len();
        let black6 = (black5 as f64).log2() as usize + 1;
        let mut black7 = vec![vec![0; black6]; black5];
        let mut black8 = vec![vec![0; black6]; black5];
        let mut black9 = vec![0; black5 + 1];

        for i in 2..=black5 { black9[i] = black9[i / 2] + 1; }
        for i in 0..black5 {
            black7[i][0] = black4[i];
            black8[i][0] = black4[i];
        }

        for j in 1..black6 {
            for i in 0..=(black5 - (1 << j)) {
                black7[i][j] = black7[i][j - 1].min(black7[i + (1 << (j - 1))][j - 1]);
                black8[i][j] = black8[i][j - 1].max(black8[i + (1 << (j - 1))][j - 1]);
            }
        }
        Self { black1: black7, black2: black8, black3: black9 }
    }

    fn black_q(&self, l: usize, r: usize) -> i32 {
        let j = self.black3[r - l + 1];
        let mn = self.black1[l][j].min(self.black1[r - (1 << j) + 1][j]);
        let mx = self.black2[l][j].max(self.black2[r - (1 << j) + 1][j]);
        mx - mn
    }
}

impl Solution {
    pub fn max_total_value(black10: Vec<i32>, mut black11: i32) -> i64 {
        let black12 = black10.len();
        let black13 = BlackTable::new(&black10);
        let mut black14 = BinaryHeap::new();
        let mut black15 = 0i64;

        for i in 0..black12 {
            black14.push((black13.black_q(0, i), 0, i));
        }

        while black11 > 0 && !black14.is_empty() {
            let (black16, l, r) = black14.pop().unwrap();
            black15 += black16 as i64;
            if l + 1 <= r {
                black14.push((black13.black_q(l + 1, r), l + 1, r));
            }
            black11 -= 1;
        }
        black15
    }
}
