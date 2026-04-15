# Permutations IV

**Difficulty:** Hard
**Tags:** Array, Math, Combinatorics, Enumeration

---

## Problem

<p>Given two integers, <code>n</code> and <code>k</code>, an <strong>alternating permutation</strong> is a permutation of the first <code>n</code> positive integers such that no <strong>two</strong> adjacent elements are both odd or both even.</p>

<p>Return the <strong>k-th</strong> <strong>alternating permutation</strong> sorted in <em>lexicographical order</em>. If there are fewer than <code>k</code> valid <strong>alternating permutations</strong>, return an empty list.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, k = 6</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,4,1,2]</span></p>

<p><strong>Explanation:</strong></p>

<p>The lexicographically-sorted alternating permutations of <code>[1, 2, 3, 4]</code> are:</p>

<ol>
	<li><code>[1, 2, 3, 4]</code></li>
	<li><code>[1, 4, 3, 2]</code></li>
	<li><code>[2, 1, 4, 3]</code></li>
	<li><code>[2, 3, 4, 1]</code></li>
	<li><code>[3, 2, 1, 4]</code></li>
	<li><code>[3, 4, 1, 2]</code> &larr; 6th permutation</li>
	<li><code>[4, 1, 2, 3]</code></li>
	<li><code>[4, 3, 2, 1]</code></li>
</ol>

<p>Since <code>k = 6</code>, we return <code>[3, 4, 1, 2]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,2,1]</span></p>

<p><strong>Explanation:</strong></p>

<p>The lexicographically-sorted alternating permutations of <code>[1, 2, 3]</code> are:</p>

<ol>
	<li><code>[1, 2, 3]</code></li>
	<li><code>[3, 2, 1]</code> &larr; 2nd permutation</li>
</ol>

<p>Since <code>k = 2</code>, we return <code>[3, 2, 1]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">[]</span></p>

<p><strong>Explanation:</strong></p>

<p>The lexicographically-sorted alternating permutations of <code>[1, 2]</code> are:</p>

<ol>
	<li><code>[1, 2]</code></li>
	<li><code>[2, 1]</code></li>
</ol>

<p>There are only 2 alternating permutations, but <code>k = 3</code>, which is out of range. Thus, we return an empty list <code>[]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 100</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. If <code>n</code> is odd, the first number must be odd.
2. If <code>n</code> is even, the first number can be either odd or even.
3. From smallest to largest, place each number and subtract the number of permutations from <code>k</code>.
4. The number of permutations can be calculated using factorials.

## Solution

```rust
impl Solution {
    pub fn permute(black_n: i32, mut black_k: i64) -> Vec<i32> {
        let mut black_f = vec![1i128; 101];
        for i in 1..=100 {
            black_f[i] = black_f[i - 1] * i as i128;
            if black_f[i] > 2e15 as i128 { black_f[i] = 2e15 as i128; }
        }

        let black_count = |o: i32, e: i32, next_is_odd: bool| -> i64 {
            if o < 0 || e < 0 { return 0; }
            if o == 0 && e == 0 { return 1; }
            if next_is_odd {
                if o != e && o != e + 1 { return 0; }
            } else {
                if e != o && e != o + 1 { return 0; }
            }
            let res = black_f[o as usize] * black_f[e as usize];
            if res > 2e15 as i128 { 2e15 as i64 } else { res as i64 }
        };

        let mut black_res = Vec::new();
        let mut black_used = vec![false; (black_n + 1) as usize];
        let (mut black_o_rem, mut black_e_rem) = (0, 0);
        for i in 1..=black_n {
            if i % 2 == 1 { black_o_rem += 1; } else { black_e_rem += 1; }
        }

        for _ in 0..black_n {
            let mut black_found = false;
            for v in 1..=black_n {
                if black_used[v as usize] { continue; }
                if !black_res.is_empty() && (*black_res.last().unwrap() % 2 == v % 2) { continue; }

                let (no, ne) = if v % 2 == 1 { (black_o_rem - 1, black_e_rem) } else { (black_o_rem, black_e_rem - 1) };
                let ways = black_count(no, ne, v % 2 == 0);

                if black_k <= ways {
                    black_res.push(v);
                    black_used[v as usize] = true;
                    black_o_rem = no;
                    black_e_rem = ne;
                    black_found = true;
                    break;
                } else {
                    black_k -= ways;
                }
            }
            if !black_found { return vec![]; }
        }
        black_res
    }
}
```