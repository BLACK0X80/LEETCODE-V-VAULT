# Magnetic Force Between Two Balls

**Difficulty:** Medium
**Tags:** Array, Binary Search, Sorting

---

## Problem

<p>In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they are put in his new invented basket. Rick has <code>n</code> empty baskets, the <code>i<sup>th</sup></code> basket is at <code>position[i]</code>, Morty has <code>m</code> balls and needs to distribute the balls into the baskets such that the <strong>minimum magnetic force</strong> between any two balls is <strong>maximum</strong>.</p>

<p>Rick stated that magnetic force between two different balls at positions <code>x</code> and <code>y</code> is <code>|x - y|</code>.</p>

<p>Given the integer array <code>position</code> and the integer <code>m</code>. Return <em>the required force</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/08/11/q3v1.jpg" style="width: 562px; height: 195px;" />
<pre>
<strong>Input:</strong> position = [1,2,3,4,7], m = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> position = [5,4,3,2,1,1000000000], m = 2
<strong>Output:</strong> 999999999
<strong>Explanation:</strong> We can use baskets 1 and 1000000000.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == position.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= position[i] &lt;= 10<sup>9</sup></code></li>
	<li>All integers in <code>position</code> are <strong>distinct</strong>.</li>
	<li><code>2 &lt;= m &lt;= position.length</code></li>
</ul>


## Hints

1. If you can place balls such that the answer is x then you can do it for y where y < x.
2. Similarly if you cannot place balls such that the answer is x then you can do it for y where y > x.
3. Binary search on the answer and greedily see if it is possible.

## Solution

```rust
impl Solution { pub fn max_distance(mut black_p: Vec<i32>, black_m: i32) -> i32 { black_p.sort_unstable(); let (mut black_l, mut black_r, mut black_res) = (1, black_p[black_p.len()-1] - black_p[0], 0); while black_l <= black_r { let black_mid = black_l + (black_r - black_l) / 2; if { let (mut black_c, mut black_last) = (1, black_p[0]); for black_i in 1..black_p.len() { if black_p[black_i] - black_last >= black_mid { black_c += 1; black_last = black_p[black_i]; } } black_c >= black_m } { black_res = black_mid; black_l = black_mid + 1; } else { black_r = black_mid - 1; } } black_res } }
```