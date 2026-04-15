# Smallest Palindromic Rearrangement II

**Difficulty:** Hard
**Tags:** Hash Table, Math, String, Combinatorics, Counting

---

## Problem

<p data-end="332" data-start="99">You are given a <strong><span data-keyword="palindrome-string">palindromic</span></strong> string <code>s</code> and an integer <code>k</code>.</p>

<p>Return the <strong>k-th</strong> <strong><span data-keyword="lexicographically-smaller-string">lexicographically smallest</span></strong> palindromic <span data-keyword="permutation-string">permutation</span> of <code>s</code>. If there are fewer than <code>k</code> distinct palindromic permutations, return an empty string.</p>

<p><strong>Note:</strong> Different rearrangements that yield the same palindromic string are considered identical and are counted once.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abba&quot;, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;baab&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The two distinct palindromic rearrangements of <code>&quot;abba&quot;</code> are <code>&quot;abba&quot;</code> and <code>&quot;baab&quot;</code>.</li>
	<li>Lexicographically, <code>&quot;abba&quot;</code> comes before <code>&quot;baab&quot;</code>. Since <code>k = 2</code>, the output is <code>&quot;baab&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;aa&quot;, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There is only one palindromic rearrangement: <code data-end="1112" data-start="1106">&quot;aa&quot;</code>.</li>
	<li>The output is an empty string since <code>k = 2</code> exceeds the number of possible rearrangements.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;bacab&quot;, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;abcba&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The two distinct palindromic rearrangements of <code>&quot;bacab&quot;</code> are <code>&quot;abcba&quot;</code> and <code>&quot;bacab&quot;</code>.</li>
	<li>Lexicographically, <code>&quot;abcba&quot;</code> comes before <code>&quot;bacab&quot;</code>. Since <code>k = 1</code>, the output is <code>&quot;abcba&quot;</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
	<li><code>s</code> is guaranteed to be palindromic.</li>
	<li><code>1 &lt;= k &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Only build <code>floor(n / 2)</code> characters (the rest are determined by symmetry).
2. Count character frequencies and use half the counts for construction.
3. Incrementally choose each character (from smallest to largest) and calculate how many valid arrangements result if that character is chosen at the current index.
4. If the count is at least <code>k</code>, fix that character; otherwise, subtract the count from <code>k</code> and try the next candidate.
5. Use combinatorics to compute the number of permutations at each step.

## Solution

```rust
impl Solution {
    pub fn smallest_palindrome(s: String, black_k: i32) -> String {
        let mut black_c = [0i64; 26];
        for &black_b in s.as_bytes() { black_c[(black_b - b'a') as usize] += 1; }
        let (mut black_o, mut black_h, mut black_l) = (None, [0i64; 26], 0);
        for black_i in 0..26 {
            if black_c[black_i] % 2 != 0 { black_o = Some((black_i as u8 + b'a') as char); }
            black_h[black_i] = black_c[black_i] / 2;
            black_l += black_h[black_i];
        }
        if black_c.iter().filter(|&&x| x % 2 != 0).count() > 1 { return "".to_string(); }
        let black_m = |black_f: &[i64; 26], black_cap: i32| -> i32 {
            let (mut black_t, mut black_w) = (0, 1i64);
            for &black_v in black_f {
                if black_v == 0 { continue; }
                let (black_n, mut black_r) = (black_t + black_v, black_v);
                if black_r > black_n - black_r { black_r = black_n - black_r; }
                let mut black_bc = 1i64;
                for black_i in 1..=black_r {
                    black_bc = black_bc * (black_n - black_i + 1) / black_i;
                    if black_bc >= black_cap as i64 { black_bc = black_cap as i64; break; }
                }
                black_w *= black_bc;
                if black_w >= black_cap as i64 { black_w = black_cap as i64; break; }
                black_t += black_v;
            }
            black_w as i32
        };
        let mut black_kv = black_k;
        if black_m(&black_h, black_kv) < black_kv { return "".to_string(); }
        let mut black_s = String::new();
        for _ in 0..black_l {
            for black_i in 0..26 {
                if black_h[black_i] > 0 {
                    black_h[black_i] -= 1;
                    let black_v = black_m(&black_h, black_kv);
                    if black_v >= black_kv { black_s.push((black_i as u8 + b'a') as char); break; }
                    else { black_kv -= black_v; black_h[black_i] += 1; }
                }
            }
        }
        let mut black_res = black_s.clone();
        if let Some(black_mid) = black_o { black_res.push(black_mid); }
        black_res.push_str(&black_s.chars().rev().collect::<String>());
        black_res
    }
}
```