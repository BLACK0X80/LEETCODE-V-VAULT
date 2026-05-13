# Minimum Number of Valid Strings to Form Target II

**Difficulty:** Hard
**Tags:** Array, String, Binary Search, Dynamic Programming, Greedy, Segment Tree, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given an array of strings <code>words</code> and a string <code>target</code>.</p>

<p>A string <code>x</code> is called <strong>valid</strong> if <code>x</code> is a <span data-keyword="string-prefix">prefix</span> of <strong>any</strong> string in <code>words</code>.</p>

<p>Return the <strong>minimum</strong> number of <strong>valid</strong> strings that can be <em>concatenated</em> to form <code>target</code>. If it is <strong>not</strong> possible to form <code>target</code>, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;abc&quot;,&quot;aaaaa&quot;,&quot;bcdef&quot;], target = &quot;aabcdabc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The target string can be formed by concatenating:</p>

<ul>
	<li>Prefix of length 2 of <code>words[1]</code>, i.e. <code>&quot;aa&quot;</code>.</li>
	<li>Prefix of length 3 of <code>words[2]</code>, i.e. <code>&quot;bcd&quot;</code>.</li>
	<li>Prefix of length 3 of <code>words[0]</code>, i.e. <code>&quot;abc&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;abababab&quot;,&quot;ab&quot;], target = &quot;ababaababa&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The target string can be formed by concatenating:</p>

<ul>
	<li>Prefix of length 5 of <code>words[0]</code>, i.e. <code>&quot;ababa&quot;</code>.</li>
	<li>Prefix of length 5 of <code>words[0]</code>, i.e. <code>&quot;ababa&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;abcdef&quot;], target = &quot;xyz&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 100</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 5 * 10<sup>4</sup></code></li>
	<li>The input is generated such that <code>sum(words[i].length) &lt;= 10<sup>5</sup></code>.</li>
	<li><code>words[i]</code> consists only of lowercase English letters.</li>
	<li><code>1 &lt;= target.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>target</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Let <code>dp[i]</code> be the minimum cost to form the prefix of length <code>i</code> of <code>target</code>.
2. Use Rabin-Karp to hash every prefix and store it in a HashSet.
3. Use Binary search to find the longest substring starting at index <code>i</code> (<code>target[i..j]</code>) that has a hash present in the HashSet.
4. Inverse Modulo precomputation can optimise hash calculation.
5. Use Lazy Segment Tree, or basic Segment Tree to update <code>dp[i..j]</code>.
6. Is it possible to use two TreeSets to update <code>dp[i..j]</code>?

## Solution

```rust
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
```