# Kth Smallest Instructions

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Combinatorics

---

## Problem

<p>Bob is standing at cell <code>(0, 0)</code>, and he wants to reach <code>destination</code>: <code>(row, column)</code>. He can only travel <strong>right</strong> and <strong>down</strong>. You are going to help Bob by providing <strong>instructions</strong> for him to reach <code>destination</code>.</p>

<p>The <strong>instructions</strong> are represented as a string, where each character is either:</p>

<ul>
	<li><code>&#39;H&#39;</code>, meaning move horizontally (go <strong>right</strong>), or</li>
	<li><code>&#39;V&#39;</code>, meaning move vertically (go <strong>down</strong>).</li>
</ul>

<p>Multiple <strong>instructions</strong> will lead Bob to <code>destination</code>. For example, if <code>destination</code> is <code>(2, 3)</code>, both <code>&quot;HHHVV&quot;</code> and <code>&quot;HVHVH&quot;</code> are valid <strong>instructions</strong>.</p>

<p>However, Bob is very picky. Bob has a lucky number <code>k</code>, and he wants the <code>k<sup>th</sup></code> <strong>lexicographically smallest instructions</strong> that will lead him to <code>destination</code>. <code>k</code> is <strong>1-indexed</strong>.</p>

<p>Given an integer array <code>destination</code> and an integer <code>k</code>, return <em>the </em><code>k<sup>th</sup></code><em> <strong>lexicographically smallest instructions</strong> that will take Bob to </em><code>destination</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex1.png" style="width: 300px; height: 229px;" /></p>

<pre>
<strong>Input:</strong> destination = [2,3], k = 1
<strong>Output:</strong> &quot;HHHVV&quot;
<strong>Explanation:</strong> All the instructions that reach (2, 3) in lexicographic order are as follows:
[&quot;HHHVV&quot;, &quot;HHVHV&quot;, &quot;HHVVH&quot;, &quot;HVHHV&quot;, &quot;HVHVH&quot;, &quot;HVVHH&quot;, &quot;VHHHV&quot;, &quot;VHHVH&quot;, &quot;VHVHH&quot;, &quot;VVHHH&quot;].
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex2.png" style="width: 300px; height: 229px;" /></strong></p>

<pre>
<strong>Input:</strong> destination = [2,3], k = 2
<strong>Output:</strong> &quot;HHVHV&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex3.png" style="width: 300px; height: 229px;" /></strong></p>

<pre>
<strong>Input:</strong> destination = [2,3], k = 3
<strong>Output:</strong> &quot;HHVVH&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>destination.length == 2</code></li>
	<li><code>1 &lt;= row, column &lt;= 15</code></li>
	<li><code>1 &lt;= k &lt;= nCr(row + column, row)</code>, where <code>nCr(a, b)</code> denotes <code>a</code> choose <code>b</code>​​​​​.</li>
</ul>


## Hints

1. There are nCr(row + column, row) possible instructions to reach (row, column).
2. Try building the instructions one step at a time. How many instructions start with "H", and how does this compare with k?

## Solution

```rust
impl Solution {
    pub fn kth_smallest_path(black_dest: Vec<i32>, mut black_k: i32) -> String {
        let (black_v_total, black_h_total) = (black_dest[0] as usize, black_dest[1] as usize);
        let mut black_comb = vec![vec![0i32; 31]; 31];
        for i in 0..31 {
            black_comb[i][0] = 1;
            for j in 1..=i { black_comb[i][j] = black_comb[i - 1][j - 1] + black_comb[i - 1][j]; }
        }

        let mut black_res = String::new();
        let (mut black_h, mut black_v) = (black_h_total, black_v_total);

        for _ in 0..(black_v_total + black_h_total) {
            if black_h > 0 {
                let black_count = black_comb[black_h + black_v - 1][black_v];
                if black_k <= black_count {
                    black_res.push('H');
                    black_h -= 1;
                } else {
                    black_res.push('V');
                    black_k -= black_count;
                    black_v -= 1;
                }
            } else {
                black_res.push('V');
                black_v -= 1;
            }
        }
        black_res
    }
}
```