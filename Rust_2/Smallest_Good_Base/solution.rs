impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: u64 = n.parse().unwrap();
        let max_m = (n as f64).log2() as u32;
        for m in (2..=max_m).rev() {
            let k = (n as f64).powf(1.0 / m as f64) as u64;
            if k >= 2 {
                let sum: u64 = (0..=m).fold(0u64, |acc, i| acc + k.pow(i));
                if sum == n { return k.to_string(); }
            }
        }
        (n - 1).to_string()
    }
}