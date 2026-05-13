# Best Sightseeing Pair

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an integer array <code>values</code> where values[i] represents the value of the <code>i<sup>th</sup></code> sightseeing spot. Two sightseeing spots <code>i</code> and <code>j</code> have a <strong>distance</strong> <code>j - i</code> between them.</p>

<p>The score of a pair (<code>i &lt; j</code>) of sightseeing spots is <code>values[i] + values[j] + i - j</code>: the sum of the values of the sightseeing spots, minus the distance between them.</p>

<p>Return <em>the maximum score of a pair of sightseeing spots</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> values = [8,1,5,2,6]
<strong>Output:</strong> 11
<strong>Explanation:</strong> i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> values = [1,2]
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= values.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= values[i] &lt;= 1000</code></li>
</ul>


## Hints

1. Can you tell the best sightseeing spot in one pass (ie. as you iterate over the input?)  What should we store or keep track of as we iterate to do this?

## Solution

```rust
impl Solution { pub fn max_score_sightseeing_pair(black_vals: Vec<i32>) -> i32 { let (mut black_max_score, mut black_prev_best) = (0, black_vals[0] + 0); for j in 1..black_vals.len() { black_max_score = black_max_score.max(black_prev_best + black_vals[j] - j as i32); black_prev_best = black_prev_best.max(black_vals[j] + j as i32); } black_max_score } }
```