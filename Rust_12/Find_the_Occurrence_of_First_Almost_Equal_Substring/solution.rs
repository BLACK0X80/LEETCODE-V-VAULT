impl Solution {
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        let s = s.as_bytes();
        let p = pattern.as_bytes();
        let n = s.len();
        let m = p.len();

        fn z_func(a: &[u8]) -> Vec<usize> {
            let n = a.len();
            let mut z = vec![0usize; n];
            z[0] = n;
            let (mut l, mut r) = (0, 0);
            for i in 1..n {
                if i < r { z[i] = (r - i).min(z[i - l]); }
                while i + z[i] < n && a[z[i]] == a[i + z[i]] { z[i] += 1; }
                if i + z[i] > r { l = i; r = i + z[i]; }
            }
            z
        }

        let mut fwd = vec![b'#'; m + 1 + n];
        fwd[..m].copy_from_slice(p);
        fwd[m] = b'#';
        fwd[m+1..].copy_from_slice(s);
        let zf = z_func(&fwd);

        let mut ps: Vec<u8> = p.iter().rev().cloned().collect();
        let mut ss: Vec<u8> = s.iter().rev().cloned().collect();
        let mut bwd = vec![b'#'; m + 1 + n];
        bwd[..m].copy_from_slice(&ps);
        bwd[m] = b'#';
        bwd[m+1..].copy_from_slice(&ss);
        let zb = z_func(&bwd);

        for i in 0..=n-m {
            let prefix = zf[m + 1 + i].min(m);
            let suffix = zb[m + 1 + (n - i - m)].min(m);
            if prefix + suffix >= m - 1 {
                return i as i32;
            }
        }
        -1
    }
}