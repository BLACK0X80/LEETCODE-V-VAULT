# Permutation Sequence

**Difficulty:** Hard
**Tags:** Math, Recursion

---

## Problem

<p>The set <code>[1, 2, 3, ...,&nbsp;n]</code> contains a total of <code>n!</code> unique permutations.</p>

<p>By listing and labeling all of the permutations in order, we get the following sequence for <code>n = 3</code>:</p>

<ol>
	<li><code>&quot;123&quot;</code></li>
	<li><code>&quot;132&quot;</code></li>
	<li><code>&quot;213&quot;</code></li>
	<li><code>&quot;231&quot;</code></li>
	<li><code>&quot;312&quot;</code></li>
	<li><code>&quot;321&quot;</code></li>
</ol>

<p>Given <code>n</code> and <code>k</code>, return the <code>k<sup>th</sup></code> permutation sequence.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> n = 3, k = 3
<strong>Output:</strong> "213"
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> n = 4, k = 9
<strong>Output:</strong> "2314"
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> n = 3, k = 1
<strong>Output:</strong> "123"
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 9</code></li>
	<li><code>1 &lt;= k &lt;= n!</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let n = n as usize;
        let mut black: Vec<i32> = (1..=n as i32).collect();
        let mut fact = vec![1usize; n + 1];
        for b in 1..=n { fact[b] = fact[b-1] * b; }
        k -= 1;
        let mut res = String::new();
        for b in (0..n).rev() {
            let idx = k as usize / fact[b];
            res.push_str(&black[idx].to_string());
            black.remove(idx);
            k %= fact[b] as i32;
        }
        res
    }
}
```