impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let r = right as usize;
        let mut black = vec![true; r + 1];
        black[0] = false; if r > 0 { black[1] = false; }
        let mut b = 2;
        while b * b <= r { if black[b] { let mut bl = b*b; while bl <= r { black[bl] = false; bl += b; } } b += 1; }
        let primes: Vec<i32> = (left as usize..=r).filter(|&b| black[b]).map(|b| b as i32).collect();
        primes.windows(2).map(|w| (w[1]-w[0], w[0], w[1])).min().map(|(_,a,b)| vec![a,b]).unwrap_or(vec![-1,-1])
    }
}