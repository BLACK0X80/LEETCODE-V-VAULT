impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut fib = vec![1i32; 32];
        for i in 2..32 { fib[i] = fib[i-1] + fib[i-2]; }

        let mut result = 0;
        let mut prev_bit = 0;
        let bits = 30;

        for i in (0..=bits).rev() {
            if n & (1 << i) != 0 {
                result += fib[i + 1];
                if prev_bit == 1 { return result; }
                prev_bit = 1;
            } else {
                prev_bit = 0;
            }
        }

        result + 1
    }
}
