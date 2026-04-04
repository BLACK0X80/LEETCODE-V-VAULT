impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 { return '0'; }
        let len = (1 << n) - 1;
        let mid = len / 2 + 1;
        if k == mid { '1' }
        else if k < mid { Self::find_kth_bit(n - 1, k) }
        else {
            let mirror = mid - (k - mid);
            let c = Self::find_kth_bit(n - 1, mirror);
            if c == '0' { '1' } else { '0' }
        }
    }
}
