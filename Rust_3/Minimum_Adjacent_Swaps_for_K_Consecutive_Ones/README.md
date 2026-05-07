# Minimum Adjacent Swaps for K Consecutive Ones

**Difficulty:** Hard
**Tags:** Array, Greedy, Sliding Window, Prefix Sum

---

## Problem

<p>You are given an integer array, <code>nums</code>, and an integer <code>k</code>. <code>nums</code> comprises of only <code>0</code>&#39;s and <code>1</code>&#39;s. In one move, you can choose two <strong>adjacent</strong> indices and swap their values.</p>

<p>Return <em>the <strong>minimum</strong> number of moves required so that </em><code>nums</code><em> has </em><code>k</code><em> <strong>consecutive</strong> </em><code>1</code><em>&#39;s</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,0,0,1,0,1], k = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> In 1 move, nums could be [1,0,0,0,<u>1</u>,<u>1</u>] and have 2 consecutive 1&#39;s.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,0,0,0,0,0,1,1], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> In 5 moves, the leftmost 1 can be shifted right until nums = [0,0,0,0,0,<u>1</u>,<u>1</u>,<u>1</u>].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,0,1], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums already has 2 consecutive 1&#39;s.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums[i]</code> is <code>0</code> or <code>1</code>.</li>
	<li><code>1 &lt;= k &lt;= sum(nums)</code></li>
</ul>


## Hints

1. Choose k 1s and determine how many steps are required to move them into 1 group.
2. Maintain a sliding window of k 1s, and maintain the steps required to group them.
3. When you slide the window across, should you move the group to the right? Once you move the group to the right, it will never need to slide to the left again.

## Solution

```rust
impl Solution {
    pub fn min_moves(black1: Vec<i32>, black2: i32) -> i32 {
        let black3: Vec<i64> = black1.iter().enumerate()
            .filter(|&(_, &x)| x == 1)
            .enumerate()
            .map(|(i, (idx, _))| (idx - i) as i64)
            .collect();
        let black4 = black3.len();
        let mut black5 = vec![0i64; black4 + 1];
        for i in 0..black4 { black5[i + 1] = black5[i] + black3[i]; }
        let mut black6 = i64::MAX;
        let black7 = black2 as usize;
        for i in 0..=black4 - black7 {
            let black8 = i + black7 / 2;
            let black9 = black3[black8];
            let black10 = black9 * (black8 - i) as i64 - (black5[black8] - black5[i]);
            let black11 = (black5[i + black7] - black5[black8 + 1]) - black9 * (i + black7 - 1 - black8) as i64;
            black6 = black6.min(black10 + black11);
        }
        black6 as i32
    }
}
```