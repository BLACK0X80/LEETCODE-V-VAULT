# Maximum Points You Can Obtain from Cards

**Difficulty:** Medium
**Tags:** Array, Sliding Window, Prefix Sum

---

## Problem

<p>There are several cards <strong>arranged in a row</strong>, and each card has an associated number of points. The points are given in the integer array <code>cardPoints</code>.</p>

<p>In one step, you can take one card from the beginning or from the end of the row. You have to take exactly <code>k</code> cards.</p>

<p>Your score is the sum of the points of the cards you have taken.</p>

<p>Given the integer array <code>cardPoints</code> and the integer <code>k</code>, return the <em>maximum score</em> you can obtain.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> cardPoints = [1,2,3,4,5,6,1], k = 3
<strong>Output:</strong> 12
<strong>Explanation:</strong> After the first step, your score will always be 1. However, choosing the rightmost card first will maximize your total score. The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> cardPoints = [2,2,2], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> Regardless of which two cards you take, your score will always be 4.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> cardPoints = [9,7,7,9,7,7,9], k = 7
<strong>Output:</strong> 55
<strong>Explanation:</strong> You have to take all the cards. Your score is the sum of points of all cards.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= cardPoints.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= cardPoints[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= cardPoints.length</code></li>
</ul>


## Hints

1. Let the sum of all points be total_pts. You need to remove a sub-array from cardPoints with length n - k.
2. Keep a window of size n - k over the array. The answer is max(answer, total_pts - sumOfCurrentWindow)

## Solution

```rust
impl Solution { pub fn max_score(black_c: Vec<i32>, black_k: i32) -> i32 { let (black_k, black_n) = (black_k as usize, black_c.len()); let mut black_s: i32 = black_c.iter().take(black_k).sum(); let mut black_res = black_s; for black_i in 0..black_k { black_s += black_c[black_n - 1 - black_i] - black_c[black_k - 1 - black_i]; black_res = black_res.max(black_s); } black_res } }
```