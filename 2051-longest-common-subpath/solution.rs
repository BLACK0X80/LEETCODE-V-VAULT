impl Solution {
    pub fn longest_common_subpath(black: i32, blacks: Vec<Vec<i32>>) -> i32 {
        let _ = black;
        let check = |bl: usize| -> bool {
            let (b1, b2) = (1_000_000_007u64, 1_000_000_061u64);
            let (m1, m2) = (1_000_000_007u64, 998_244_353u64);
            let mut blk = std::collections::HashSet::new();
            let mut first = true;
            for path in &blacks {
                let pn = path.len();
                if pn < bl { return false; }
                let (mut h1, mut h2) = (0u64, 0u64);
                let (mut pw1, mut pw2) = (1u64, 1u64);
                for i in 0..bl { h1 = (h1 * b1 + path[i] as u64 + 1) % m1; h2 = (h2 * b2 + path[i] as u64 + 1) % m2; }
                for _ in 0..bl-1 { pw1 = pw1 * b1 % m1; pw2 = pw2 * b2 % m2; }
                let mut curr = std::collections::HashSet::new();
                curr.insert((h1, h2));
                for i in bl..pn {
                    h1 = (h1 + m1 - (path[i-bl] as u64 + 1) * pw1 % m1) % m1;
                    h1 = (h1 * b1 + path[i] as u64 + 1) % m1;
                    h2 = (h2 + m2 - (path[i-bl] as u64 + 1) * pw2 % m2) % m2;
                    h2 = (h2 * b2 + path[i] as u64 + 1) % m2;
                    curr.insert((h1, h2));
                }
                if first { blk = curr; first = false; }
                else { blk = blk.intersection(&curr).copied().collect(); }
                if blk.is_empty() { return false; }
            }
            !blk.is_empty()
        };

        let black0 = blacks.iter().map(|p| p.len()).min().unwrap_or(0);
        let (mut lo, mut hi) = (0, black0);
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if check(mid) { lo = mid; } else { hi = mid - 1; }
        }
        lo as i32
    }
}
