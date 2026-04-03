impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let t = target.as_bytes();
        let n = t.len();

        fn z_func(a: &[u8]) -> Vec<usize> {
            let n = a.len();
            let mut z = vec![0usize; n];
            z[0] = n;
            let (mut l, mut r) = (0usize, 0usize);
            for i in 1..n {
                if i < r { z[i] = (r - i).min(z[i - l]); }
                while i + z[i] < n && a[z[i]] == a[i + z[i]] { z[i] += 1; }
                if i + z[i] > r { l = i; r = i + z[i]; }
            }
            z
        }

        let mut reach = vec![0usize; n];
        for word in &words {
            let w = word.as_bytes();
            let mut combined = w.to_vec();
            combined.push(b'#');
            combined.extend_from_slice(t);
            let z = z_func(&combined);
            let off = w.len() + 1;
            for i in 0..n {
                let matched = z[off + i].min(w.len());
                if i + matched > reach[i] { reach[i] = i + matched; }
            }
        }

        let mut ans = 0;
        let mut cur = 0;
        let mut far = 0;
        for i in 0..n {
            far = far.max(reach[i]);
            if i == cur {
                if far <= i { return -1; }
                ans += 1;
                cur = far;
            }
        }
        ans
    }
}
