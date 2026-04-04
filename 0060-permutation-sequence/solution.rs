impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let n = n as usize;
        let mut black: Vec<i32> = (1..=n as i32).collect();
        let mut fact = vec![1usize; n + 1];
        for b in 1..=n { fact[b] = fact[b-1] * b; }
        k -= 1;
        let mut res = String::new();
        for b in (0..n).rev() {
            let idx = k as usize / fact[b];
            res.push_str(&black[idx].to_string());
            black.remove(idx);
            k %= fact[b] as i32;
        }
        res
    }
}
