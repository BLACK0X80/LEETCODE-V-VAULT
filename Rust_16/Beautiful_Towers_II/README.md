# Beautiful Towers II

**Difficulty:** Medium
**Tags:** Array, Stack, Monotonic Stack

---

## Problem

<p>You are given a <strong>0-indexed</strong> array <code>maxHeights</code> of <code>n</code> integers.</p>

<p>You are tasked with building <code>n</code> towers in the coordinate line. The <code>i<sup>th</sup></code> tower is built at coordinate <code>i</code> and has a height of <code>heights[i]</code>.</p>

<p>A configuration of towers is <strong>beautiful</strong> if the following conditions hold:</p>

<ol>
	<li><code>1 &lt;= heights[i] &lt;= maxHeights[i]</code></li>
	<li><code>heights</code> is a <strong>mountain</strong> array.</li>
</ol>

<p>Array <code>heights</code> is a <strong>mountain</strong> if there exists an index <code>i</code> such that:</p>

<ul>
	<li>For all <code>0 &lt; j &lt;= i</code>, <code>heights[j - 1] &lt;= heights[j]</code></li>
	<li>For all <code>i &lt;= k &lt; n - 1</code>, <code>heights[k + 1] &lt;= heights[k]</code></li>
</ul>

<p>Return <em>the <strong>maximum possible sum of heights</strong> of a beautiful configuration of towers</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> maxHeights = [5,3,4,1,1]
<strong>Output:</strong> 13
<strong>Explanation:</strong> One beautiful configuration with a maximum sum is heights = [5,3,3,1,1]. This configuration is beautiful since:
- 1 &lt;= heights[i] &lt;= maxHeights[i]  
- heights is a mountain of peak i = 0.
It can be shown that there exists no other beautiful configuration with a sum of heights greater than 13.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> maxHeights = [6,5,3,9,2,7]
<strong>Output:</strong> 22
<strong>Explanation:</strong> One beautiful configuration with a maximum sum is heights = [3,3,3,9,2,2]. This configuration is beautiful since:
- 1 &lt;= heights[i] &lt;= maxHeights[i]
- heights is a mountain of peak i = 3.
It can be shown that there exists no other beautiful configuration with a sum of heights greater than 22.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> maxHeights = [3,2,5,5,2,3]
<strong>Output:</strong> 18
<strong>Explanation:</strong> One beautiful configuration with a maximum sum is heights = [2,2,5,5,2,2]. This configuration is beautiful since:
- 1 &lt;= heights[i] &lt;= maxHeights[i]
- heights is a mountain of peak i = 2. 
Note that, for this configuration, i = 3 can also be considered a peak.
It can be shown that there exists no other beautiful configuration with a sum of heights greater than 18.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == maxHeights.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= maxHeights[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Try all the possible indices <code>i</code> as the peak.
2. Let <code>left[i]</code> be the maximum sum of heights for the prefix <code>0, …, i</code> when index <code>i</code> is the peak.
3. Let <code>right[i]</code> be the maximum sum of heights for suffix <code>i, …, (n - 1)</code> when <code>i</code> is the peak
4. Compute values of <code>left[i]</code> from left to right using DP.
For each <code>i</code> from <code>0</code> to <code>n - 1</code>, <code>left[i] = maxHeights * (i - j) + answer[j]</code>, where <code>j</code> is the rightmost index to the left of <code>i</code> such that <code>maxHeights[j] < maxHeights[i] </code>.
5. For each <code>i</code> from <code>n - 1</code> to <code>0</code>, <code>right[i] = maxHeights * (j - i) + answer[j]</code>, where <code>j</code> is the leftmost index to the right of <code>i</code> such that <code>maxHeights[j] < maxHeights[i] </code>.

## Solution

```rust
impl Solution { pub fn maximum_sum_of_heights(black_h: Vec<i32>) -> i64 { let black_n = black_h.len(); let black_f = |black_v: &Vec<i32>| { let (mut black_s, mut black_res, mut black_cur) = (vec![(-1isize, 0i64)], vec![0i64; black_n], 0i64); for black_i in 0..black_n { while let Some(&(black_p, _)) = black_s.last() { if black_p != -1 && black_v[black_p as usize] >= black_v[black_i] { black_cur -= black_s.pop().unwrap().1; } else { break; } } let black_prev = black_s.last().unwrap().0; let black_val = (black_i as isize - black_prev) as i64 * black_v[black_i] as i64; black_cur += black_val; black_res[black_i] = black_cur; black_s.push((black_i as isize, black_val)); } black_res }; let (black_l, mut black_rv) = (black_f(&black_h), black_h.clone()); black_rv.reverse(); let mut black_r = black_f(&black_rv); black_r.reverse(); (0..black_n).map(|black_i| black_l[black_i] + black_r[black_i] - black_h[black_i] as i64).max().unwrap() } }
```