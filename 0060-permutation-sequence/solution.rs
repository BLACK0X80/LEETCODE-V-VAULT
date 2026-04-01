impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let n = n as usize;
        let mut digits: Vec<i32> = (1..=n as i32).collect();
        let mut factorial = vec![1usize; n + 1];

        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i;
        }

        k -= 1;
        let mut result = String::new();

        for i in (0..n).rev() {
            let idx = k as usize / factorial[i];
            result.push_str(&digits[idx].to_string());
            digits.remove(idx);
            k %= factorial[i] as i32;
        }

        result
    }
}
