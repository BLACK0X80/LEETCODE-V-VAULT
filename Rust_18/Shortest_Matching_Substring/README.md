# Shortest Matching Substring

**Difficulty:** Hard
**Tags:** Two Pointers, String, Binary Search, String Matching

---

## Problem

<p>You are given a string <code>s</code> and a pattern string <code>p</code>, where <code>p</code> contains <strong>exactly two</strong> <code>&#39;*&#39;</code> characters.</p>

<p>The <code>&#39;*&#39;</code> in <code>p</code> matches any sequence of zero or more characters.</p>

<p>Return the length of the <strong>shortest</strong> <span data-keyword="substring">substring</span> in <code>s</code> that matches <code>p</code>. If there is no such substring, return -1.</p>
<strong>Note:</strong> The empty substring is considered valid.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abaacbaecebce&quot;, p = &quot;ba*c*ce&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p>The shortest matching substring of <code>p</code> in <code>s</code> is <code>&quot;<u><strong>ba</strong></u>e<u><strong>c</strong></u>eb<u><strong>ce</strong></u>&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;baccbaadbc&quot;, p = &quot;cc*baa*adb&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no matching substring in <code>s</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;a&quot;, p = &quot;**&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The empty substring is the shortest matching substring.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;madlogic&quot;, p = &quot;*adlogi*&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The shortest matching substring of <code>p</code> in <code>s</code> is <code>&quot;<strong><u>adlogi</u></strong>&quot;</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= p.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> contains only lowercase English letters.</li>
	<li><code>p</code> contains only lowercase English letters and exactly two <code>&#39;*&#39;</code>.</li>
</ul>


## Hints

1. The pattern string <code>p</code> can be divided into three segments.
2. Use the KMP algorithm to locate all occurrences of each segment in <code>s</code>.

## Solution

```rust
impl Solution {
    pub fn shortest_matching_substring(s: String, p: String) -> i32 {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let stars: Vec<usize> = p.iter().enumerate().filter(|&(_,&c)| c == b'*').map(|(i,_)| i).collect();
        let (i1, i2) = (stars[0], stars[1]);
        let p1 = &p[..i1];
        let p2 = &p[i1+1..i2];
        let p3 = &p[i2+1..];

        fn kmp_fail(pat: &[u8]) -> Vec<usize> {
            let m = pat.len();
            let mut f = vec![0usize; m];
            let mut j = 0usize;
            for i in 1..m {
                while j > 0 && pat[i] != pat[j] { j = f[j-1]; }
                if pat[i] == pat[j] { j += 1; }
                f[i] = j;
            }
            f
        }

        fn kmp_search(text: &[u8], pat: &[u8]) -> Vec<usize> {
            if pat.is_empty() { return (0..=text.len()).collect(); }
            let f = kmp_fail(pat);
            let mut res = vec![];
            let mut j = 0usize;
            for (i, &c) in text.iter().enumerate() {
                while j > 0 && c != pat[j] { j = f[j-1]; }
                if c == pat[j] { j += 1; }
                if j == pat.len() { res.push(i + 1 - pat.len()); j = f[j-1]; }
            }
            res
        }

        let pos1 = kmp_search(s, p1);
        let pos2 = kmp_search(s, p2);
        let pos3 = kmp_search(s, p3);

        let n = s.len();
        let mut ans = i32::MAX;
        let (mut j2, mut j3) = (0usize, 0usize);

        for &start in &pos1 {
            let after1 = start + p1.len();
            while j2 < pos2.len() && pos2[j2] < after1 { j2 += 1; }
            if j2 >= pos2.len() { break; }
            let after2 = pos2[j2] + p2.len();
            while j3 < pos3.len() && pos3[j3] < after2 { j3 += 1; }
            if j3 >= pos3.len() { break; }
            let end = pos3[j3] + p3.len();
            ans = ans.min((end - start) as i32);
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}
```