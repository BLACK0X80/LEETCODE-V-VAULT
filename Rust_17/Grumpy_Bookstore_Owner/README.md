# Grumpy Bookstore Owner

**Difficulty:** Medium
**Tags:** Array, Sliding Window

---

## Problem

<p>There is a bookstore owner that has a store open for <code>n</code> minutes. You are given an integer array <code>customers</code> of length <code>n</code> where <code>customers[i]</code> is the number of the customers that enter the store at the start of the <code>i<sup>th</sup></code> minute and all those customers leave after the end of that minute.</p>

<p>During certain minutes, the bookstore owner is grumpy. You are given a binary array grumpy where <code>grumpy[i]</code> is <code>1</code> if the bookstore owner is grumpy during the <code>i<sup>th</sup></code> minute, and is <code>0</code> otherwise.</p>

<p>When the bookstore owner is grumpy, the customers entering during that minute are not <strong>satisfied</strong>. Otherwise, they are satisfied.</p>

<p>The bookstore owner knows a secret technique to remain <strong>not grumpy</strong> for <code>minutes</code> consecutive minutes, but this technique can only be used <strong>once</strong>.</p>

<p>Return the <strong>maximum</strong> number of customers that can be <em>satisfied</em> throughout the day.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], minutes = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">16</span></p>

<p><strong>Explanation:</strong></p>

<p>The bookstore owner keeps themselves not grumpy for the last 3 minutes.</p>

<p>The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 + 5 = 16.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">customers = [1], grumpy = [0], minutes = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == customers.length == grumpy.length</code></li>
	<li><code>1 &lt;= minutes &lt;= n &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= customers[i] &lt;= 1000</code></li>
	<li><code>grumpy[i]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. Say the store owner uses their power in minute 1 to X and we have some answer A.  If they instead use their power from minute 2 to X+1, we only have to use data from minutes 1, 2, X and X+1 to update our answer A.

## Solution

```rust
impl Solution { pub fn max_satisfied(black_c: Vec<i32>, black_g: Vec<i32>, black_m: i32) -> i32 { let (black_m, mut black_base, mut black_cur, mut black_max) = (black_m as usize, 0, 0, 0); for (black_i, (&black_cv, &black_gv)) in black_c.iter().zip(black_g.iter()).enumerate() { if black_gv == 0 { black_base += black_cv; } else { if black_i < black_m { black_cur += black_cv; } } if black_i >= black_m && black_g[black_i - black_m] == 1 { black_cur -= black_c[black_i - black_m]; } if black_i >= black_m && black_gv == 1 { black_cur += black_cv; } if black_i >= black_m - 1 { black_max = black_max.max(black_cur); } } black_base + black_max } }
```