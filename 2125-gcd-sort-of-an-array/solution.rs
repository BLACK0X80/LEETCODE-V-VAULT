impl Solution {
    pub fn gcd_sort(black1: Vec<i32>) -> bool {
        let black2 = *black1.iter().max().unwrap() as usize;
        let mut black3: Vec<usize> = (0..=black2).collect();
        
        let mut black4 = vec![0; black2 + 1];
        for &x in &black1 { black4[x as usize] = 1; }

        let mut black5 = vec![true; black2 + 1];
        for i in 2..=black2 {
            if black5[i] {
                for j in (i * 2..=black2).step_by(i) {
                    black5[j] = false;
                    if black4[j] == 1 {
                        Self::black_union(&mut black3, i, j);
                    }
                }
                if black4[i] == 1 { Self::black_union(&mut black3, i, i); }
            }
        }

        let mut black6 = black1.clone();
        black6.sort();

        for i in 0..black1.len() {
            if black1[i] != black6[i] {
                if Self::black_find(&mut black3, black1[i] as usize) != 
                   Self::black_find(&mut black3, black6[i] as usize) {
                    return false;
                }
            }
        }
        true
    }

    fn black_find(b: &mut Vec<usize>, i: usize) -> usize {
        if b[i] == i { i }
        else { b[i] = Self::black_find(b, b[i]); b[i] }
    }

    fn black_union(b: &mut Vec<usize>, i: usize, j: usize) {
        let root_i = Self::black_find(b, i);
        let root_j = Self::black_find(b, j);
        if root_i != root_j { b[root_i] = root_j; }
    }
}
