use std::collections::BTreeMap;

impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut odd_next = vec![0usize; n];
        let mut even_next = vec![0usize; n];
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();

        for i in (0..n).rev() {
            let v = arr[i];
            odd_next[i] = map.range(v..).next().map_or(n, |(_, &j)| j);
            even_next[i] = map.range(..=v).next_back().map_or(n, |(_, &j)| j);
            map.insert(v, i);
        }

        let mut odd = vec![false; n];
        let mut even = vec![false; n];
        odd[n - 1] = true;
        even[n - 1] = true;

        for i in (0..n - 1).rev() {
            if odd_next[i] < n { odd[i] = even[odd_next[i]]; }
            if even_next[i] < n { even[i] = odd[even_next[i]]; }
        }

        odd.iter().filter(|&&x| x).count() as i32
    }
}