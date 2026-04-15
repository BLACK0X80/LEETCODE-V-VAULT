# Find Beautiful Indices in the Given Array II

**Difficulty:** Hard
**Tags:** Two Pointers, String, Binary Search, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given a <strong>0-indexed</strong> string <code>s</code>, a string <code>a</code>, a string <code>b</code>, and an integer <code>k</code>.</p>

<p>An index <code>i</code> is <strong>beautiful</strong> if:</p>

<ul>
	<li><code>0 &lt;= i &lt;= s.length - a.length</code></li>
	<li><code>s[i..(i + a.length - 1)] == a</code></li>
	<li>There exists an index <code>j</code> such that:
	<ul>
		<li><code>0 &lt;= j &lt;= s.length - b.length</code></li>
		<li><code>s[j..(j + b.length - 1)] == b</code></li>
		<li><code>|j - i| &lt;= k</code></li>
	</ul>
	</li>
</ul>

<p>Return <em>the array that contains beautiful indices in <strong>sorted order from smallest to largest</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;isawsquirrelnearmysquirrelhouseohmy&quot;, a = &quot;my&quot;, b = &quot;squirrel&quot;, k = 15
<strong>Output:</strong> [16,33]
<strong>Explanation:</strong> There are 2 beautiful indices: [16,33].
- The index 16 is beautiful as s[16..17] == &quot;my&quot; and there exists an index 4 with s[4..11] == &quot;squirrel&quot; and |16 - 4| &lt;= 15.
- The index 33 is beautiful as s[33..34] == &quot;my&quot; and there exists an index 18 with s[18..25] == &quot;squirrel&quot; and |33 - 18| &lt;= 15.
Thus we return [16,33] as the result.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcd&quot;, a = &quot;a&quot;, b = &quot;a&quot;, k = 4
<strong>Output:</strong> [0]
<strong>Explanation:</strong> There is 1 beautiful index: [0].
- The index 0 is beautiful as s[0..0] == &quot;a&quot; and there exists an index 0 with s[0..0] == &quot;a&quot; and |0 - 0| &lt;= 4.
Thus we return [0] as the result.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>1 &lt;= a.length, b.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>s</code>, <code>a</code>, and <code>b</code> contain only lowercase English letters.</li>
</ul>


## Hints

1. Use KMP or string hashing.

## Solution

```rust
impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        fn kmp_search(text: &[u8], pat: &[u8]) -> Vec<usize> {
            let m = pat.len();
            let mut fail = vec![0usize; m];
            let mut j = 0usize;
            for i in 1..m {
                while j > 0 && pat[i] != pat[j] { j = fail[j-1]; }
                if pat[i] == pat[j] { j += 1; }
                fail[i] = j;
            }
            let mut res = vec![];
            j = 0;
            for (i, &c) in text.iter().enumerate() {
                while j > 0 && c != pat[j] { j = fail[j-1]; }
                if c == pat[j] { j += 1; }
                if j == m { res.push(i + 1 - m); j = fail[j-1]; }
            }
            res
        }
        let s = s.as_bytes();
        let pos_a = kmp_search(s, a.as_bytes());
        let pos_b = kmp_search(s, b.as_bytes());
        let k = k as usize;
        let mut res = vec![];
        let mut bi = 0usize;
        for &i in &pos_a {
            while bi < pos_b.len() && pos_b[bi] + k < i { bi += 1; }
            if bi < pos_b.len() && pos_b[bi] <= i + k {
                res.push(i as i32);
            }
        }
        res
    }
}
```