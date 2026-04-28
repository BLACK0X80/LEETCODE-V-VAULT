# Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given a rectangular cake of size <code>h x w</code> and two arrays of integers <code>horizontalCuts</code> and <code>verticalCuts</code> where:</p>

<ul>
	<li><code>horizontalCuts[i]</code> is the distance from the top of the rectangular cake to the <code>i<sup>th</sup></code> horizontal cut and similarly, and</li>
	<li><code>verticalCuts[j]</code> is the distance from the left of the rectangular cake to the <code>j<sup>th</sup></code> vertical cut.</li>
</ul>

<p>Return <em>the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays</em> <code>horizontalCuts</code> <em>and</em> <code>verticalCuts</code>. Since the answer can be a large number, return this <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_2.png" style="width: 225px; height: 240px;" />
<pre>
<strong>Input:</strong> h = 5, w = 4, horizontalCuts = [1,2,4], verticalCuts = [1,3]
<strong>Output:</strong> 4 
<strong>Explanation:</strong> The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green piece of cake has the maximum area.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_3.png" style="width: 225px; height: 240px;" />
<pre>
<strong>Input:</strong> h = 5, w = 4, horizontalCuts = [3,1], verticalCuts = [1]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green and yellow pieces of cake have the maximum area.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> h = 5, w = 4, horizontalCuts = [3], verticalCuts = [3]
<strong>Output:</strong> 9
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= h, w &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= horizontalCuts.length &lt;= min(h - 1, 10<sup>5</sup>)</code></li>
	<li><code>1 &lt;= verticalCuts.length &lt;= min(w - 1, 10<sup>5</sup>)</code></li>
	<li><code>1 &lt;= horizontalCuts[i] &lt; h</code></li>
	<li><code>1 &lt;= verticalCuts[i] &lt; w</code></li>
	<li>All the elements in <code>horizontalCuts</code> are distinct.</li>
	<li>All the elements in <code>verticalCuts</code> are distinct.</li>
</ul>


## Hints

1. Sort the arrays, then compute the maximum difference between two consecutive elements for horizontal cuts and vertical cuts.
2. The answer is the product of these maximum values in horizontal cuts and vertical cuts.

## Solution

```rust
impl Solution { pub fn max_area(black_h: i32, black_w: i32, mut black_hc: Vec<i32>, mut black_vc: Vec<i32>) -> i32 { black_hc.sort_unstable(); black_vc.sort_unstable(); let black_mh = black_hc.windows(2).map(|black_w| black_w[1] - black_w[0]).chain(std::iter::once(black_hc[0])).chain(std::iter::once(black_h - black_hc.last().unwrap())).max().unwrap() as i64; let black_mw = black_vc.windows(2).map(|black_w| black_w[1] - black_w[0]).chain(std::iter::once(black_vc[0])).chain(std::iter::once(black_w - black_vc.last().unwrap())).max().unwrap() as i64; ((black_mh * black_mw) % 1_000_000_007) as i32 } }
```