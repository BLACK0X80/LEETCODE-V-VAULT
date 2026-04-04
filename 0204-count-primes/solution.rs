impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n < 2 { return 0; }
        let mut black = vec![true; n];
        black[0] = false; black[1] = false;
        let mut b = 2;
        while b * b < n {
            if black[b] { let mut bl = b*b; while bl < n { black[bl] = false; bl += b; } }
            b += 1;
        }
        black.iter().filter(|&&b| b).count() as i32
    }
}
