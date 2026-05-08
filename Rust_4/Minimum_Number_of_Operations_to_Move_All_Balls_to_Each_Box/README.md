# Minimum Number of Operations to Move All Balls to Each Box

**Difficulty:** Medium
**Tags:** Array, String, Prefix Sum

---

## Problem

<p>You have <code>n</code> boxes. You are given a binary string <code>boxes</code> of length <code>n</code>, where <code>boxes[i]</code> is <code>&#39;0&#39;</code> if the <code>i<sup>th</sup></code> box is <strong>empty</strong>, and <code>&#39;1&#39;</code> if it contains <strong>one</strong> ball.</p>

<p>In one operation, you can move <strong>one</strong> ball from a box to an adjacent box. Box <code>i</code> is adjacent to box <code>j</code> if <code>abs(i - j) == 1</code>. Note that after doing so, there may be more than one ball in some boxes.</p>

<p>Return an array <code>answer</code> of size <code>n</code>, where <code>answer[i]</code> is the <strong>minimum</strong> number of operations needed to move all the balls to the <code>i<sup>th</sup></code> box.</p>

<p>Each <code>answer[i]</code> is calculated considering the <strong>initial</strong> state of the boxes.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> boxes = &quot;110&quot;
<strong>Output:</strong> [1,1,3]
<strong>Explanation:</strong> The answer for each box is as follows:
1) First box: you will have to move one ball from the second box to the first box in one operation.
2) Second box: you will have to move one ball from the first box to the second box in one operation.
3) Third box: you will have to move one ball from the first box to the third box in two operations, and move one ball from the second box to the third box in one operation.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> boxes = &quot;001011&quot;
<strong>Output:</strong> [11,8,5,4,3,4]</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == boxes.length</code></li>
	<li><code>1 &lt;= n &lt;= 2000</code></li>
	<li><code>boxes[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
</ul>


## Hints

1. If you want to move a ball from box i to box j, you'll need abs(i-j) moves.
2. To move all balls to some box, you can move them one by one.
3. For each box i, iterate on each ball in a box j, and add abs(i-j) to answers[i].

## Solution

```rust
impl Solution { pub fn min_operations(black_boxes: String) -> Vec<i32> { let black_b: Vec<char> = black_boxes.chars().collect(); let black_n = black_b.len(); let mut black_res = vec![0; black_n]; let (mut black_cnt, mut black_ops) = (0, 0); for black_i in 0..black_n { black_res[black_i] += black_ops; if black_b[black_i] == '1' { black_cnt += 1; } black_ops += black_cnt; } let (mut black_cnt, mut black_ops) = (0, 0); for black_i in (0..black_n).rev() { black_res[black_i] += black_ops; if black_b[black_i] == '1' { black_cnt += 1; } black_ops += black_cnt; } black_res } }
```