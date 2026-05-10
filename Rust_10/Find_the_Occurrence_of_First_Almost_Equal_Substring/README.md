# Find the Occurrence of First Almost Equal Substring

**Difficulty:** Hard
**Tags:** String, String Matching

---

## Problem

<p>You are given two strings <code>s</code> and <code>pattern</code>.</p>

<p>A string <code>x</code> is called <strong>almost equal</strong> to <code>y</code> if you can change <strong>at most</strong> one character in <code>x</code> to make it <em>identical</em> to <code>y</code>.</p>

<p>Return the <strong>smallest</strong> <em>starting index</em> of a <span data-keyword="substring-nonempty">substring</span> in <code>s</code> that is <strong>almost equal</strong> to <code>pattern</code>. If no such index exists, return <code>-1</code>.</p>
A <strong>substring</strong> is a contiguous <b>non-empty</b> sequence of characters within a string.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abcdefg&quot;, pattern = &quot;bcdffg&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The substring <code>s[1..6] == &quot;bcdefg&quot;</code> can be converted to <code>&quot;bcdffg&quot;</code> by changing <code>s[4]</code> to <code>&quot;f&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;ababbababa&quot;, pattern = &quot;bacaba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The substring <code>s[4..9] == &quot;bababa&quot;</code> can be converted to <code>&quot;bacaba&quot;</code> by changing <code>s[6]</code> to <code>&quot;c&quot;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abcd&quot;, pattern = &quot;dba&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;dde&quot;, pattern = &quot;d&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= pattern.length &lt; s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> and <code>pattern</code> consist only of lowercase English letters.</li>
</ul>

<p>&nbsp;</p>
<strong>Follow-up:</strong> Could you solve the problem if <strong>at most</strong> <code>k</code> <strong>consecutive</strong> characters can be changed?

## Hints

1. Let <code>dp1[i]</code> represent the maximum length of a substring of <code>s</code> starting at index <code>i</code> that is also a prefix of <code>pattern</code>.
2. Let <code>dp2[i]</code> represent the maximum length of a substring of <code>s</code> ending at index <code>i</code> that is also a suffix of <code>pattern</code>.
3. Consider a window of size <code>pattern.length</code>. If <code>dp1[i] + i == i + pattern.length - 1 - dp2[i + pattern.length - 1]</code>, what does this signify?

## Solution

```rust
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
```