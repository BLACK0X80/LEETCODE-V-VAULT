impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        const MOD: u64 = (1 << 61) - 1;
        const BASE: u64 = 131;

        let mul = |a: u64, b: u64| -> u64 {
            let v = a as u128 * b as u128;
            let v = (v >> 61) + (v & MOD as u128);
            if v >= MOD as u128 { v as u64 - MOD } else { v as u64 }
        };

        let check = |len: usize| -> usize {
            let mut pw = 1u64;
            for _ in 0..len { pw = mul(pw, BASE); }
            let mut h = 0u64;
            for i in 0..len { h = (mul(h, BASE) + b[i] as u64) % MOD; }
            let mut seen = std::collections::HashMap::new();
            seen.entry(h).or_insert_with(Vec::new).push(0usize);
            for i in 1..=n-len {
                h = (mul(h, BASE) + MOD * 2 + b[i+len-1] as u64 - mul(pw, b[i-1] as u64)) % MOD;
                let v = seen.entry(h).or_insert_with(Vec::new);
                for &j in v.iter() {
                    if b[i..i+len] == b[j..j+len] { return i; }
                }
                v.push(i);
            }
            usize::MAX
        };

        let (mut lo, mut hi) = (0, n - 1);
        let mut start = usize::MAX;
        let mut best = 0;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            let pos = check(mid);
            if pos != usize::MAX { start = pos; best = mid; lo = mid + 1; }
            else if hi == 0 { break; } else { hi = mid - 1; }
        }
        if best == 0 { return "".to_string(); }
        String::from_utf8(b[start..start+best].to_vec()).unwrap()
    }
}