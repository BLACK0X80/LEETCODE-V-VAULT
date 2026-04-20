impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let s = s.as_bytes();
        let n = s.len();
        let mut prev2 = 1u64;
        let mut prev1 = if s[0] == b'*' { 9 } else if s[0] == b'0' { 0 } else { 1 };

        for i in 1..n {
            let cur = s[i];
            let pre = s[i-1];

            let single: u64 = match cur {
                b'*' => 9,
                b'0' => 0,
                _    => 1,
            };

            let double: u64 = match (pre, cur) {
                (b'*', b'*') => 15,
                (b'*', c)    => if c <= b'6' { 2 } else { 1 },
                (b'1', b'*') => 9,
                (b'2', b'*') => 6,
                (b'1', _)    => 1,
                (b'2', c)    => if c <= b'6' { 1 } else { 0 },
                _            => 0,
            };

            let next = (single * prev1 + double * prev2) % MOD;
            prev2 = prev1;
            prev1 = next;
        }

        prev1 as i32
    }
}