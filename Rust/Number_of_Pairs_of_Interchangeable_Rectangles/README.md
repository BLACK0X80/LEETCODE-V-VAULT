# Number of Pairs of Interchangeable Rectangles

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Counting, Number Theory

---

## Problem

<p>You are given <code>n</code> rectangles represented by a <strong>0-indexed</strong> 2D integer array <code>rectangles</code>, where <code>rectangles[i] = [width<sub>i</sub>, height<sub>i</sub>]</code> denotes the width and height of the <code>i<sup>th</sup></code> rectangle.</p>

<p>Two rectangles <code>i</code> and <code>j</code> (<code>i &lt; j</code>) are considered <strong>interchangeable</strong> if they have the <strong>same</strong> width-to-height ratio. More formally, two rectangles are <strong>interchangeable</strong> if <code>width<sub>i</sub>/height<sub>i</sub> == width<sub>j</sub>/height<sub>j</sub></code> (using decimal division, not integer division).</p>

<p>Return <em>the <strong>number</strong> of pairs of <strong>interchangeable</strong> rectangles in </em><code>rectangles</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> rectangles = [[4,8],[3,6],[10,20],[15,30]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The following are the interchangeable pairs of rectangles by index (0-indexed):
- Rectangle 0 with rectangle 1: 4/8 == 3/6.
- Rectangle 0 with rectangle 2: 4/8 == 10/20.
- Rectangle 0 with rectangle 3: 4/8 == 15/30.
- Rectangle 1 with rectangle 2: 3/6 == 10/20.
- Rectangle 1 with rectangle 3: 3/6 == 15/30.
- Rectangle 2 with rectangle 3: 10/20 == 15/30.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> rectangles = [[4,5],[7,8]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no interchangeable pairs of rectangles.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == rectangles.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>rectangles[i].length == 2</code></li>
	<li><code>1 &lt;= width<sub>i</sub>, height<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Store the rectangle height and width ratio in a hashmap.
2. Traverse the ratios, and for each ratio, use the frequency of the ratio to add to the total pair count.

## Solution

```rust
impl Solution { pub fn interchangeable_rectangles(black_rects: Vec<Vec<i32>>) -> i64 { let mut black_map = std::collections::HashMap::new(); let mut black_res = 0; fn black_gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { black_gcd(b, a % b) } } for black_r in black_rects { let black_g = black_gcd(black_r[0], black_r[1]); let black_ratio = (black_r[0]/black_g, black_r[1]/black_g); let black_c = black_map.entry(black_ratio).or_insert(0i64); black_res += *black_c; *black_c += 1; } black_res } }
```