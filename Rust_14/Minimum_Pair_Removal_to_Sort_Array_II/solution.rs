use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return 0; }
        let mut vals: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let mut nxt: Vec<usize> = (1..=n).collect();
        let mut prv: Vec<usize> = (0..n).map(|i| if i == 0 { 0 } else { i - 1 }).collect();
        let mut active = vec![true; n];

        let mut bad = 0i32;
        for i in 0..n-1 { if vals[i] > vals[i+1] { bad += 1; } }

        let mut heap: BTreeSet<(i64, usize)> = BTreeSet::new();
        for i in 0..n-1 { heap.insert((vals[i] + vals[i+1], i)); }

        let mut ops = 0;
        while bad > 0 {
            let &(_, i) = heap.iter().next().unwrap();
            let j = nxt[i];

            heap.remove(&(vals[i] + vals[j], i));

            let pi = prv[i];
            let nj = nxt[j];

            if i != pi && active[pi] {
                if vals[pi] > vals[i] { bad -= 1; }
                heap.remove(&(vals[pi] + vals[i], pi));
            }
            if vals[i] > vals[j] { bad -= 1; }
            if nj < n {
                if vals[j] > vals[nj] { bad -= 1; }
                heap.remove(&(vals[j] + vals[nj], j));
            }

            vals[i] += vals[j];
            active[j] = false;
            nxt[i] = nj;
            if nj < n { prv[nj] = i; }

            if i != pi && active[pi] {
                if vals[pi] > vals[i] { bad += 1; }
                heap.insert((vals[pi] + vals[i], pi));
            }
            if nj < n {
                if vals[i] > vals[nj] { bad += 1; }
                heap.insert((vals[i] + vals[nj], i));
            }

            ops += 1;
        }
        ops
    }
}