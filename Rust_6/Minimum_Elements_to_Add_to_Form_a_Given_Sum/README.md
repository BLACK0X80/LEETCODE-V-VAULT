# Minimum Elements to Add to Form a Given Sum

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>limit</code> and <code>goal</code>. The array <code>nums</code> has an interesting property that <code>abs(nums[i]) &lt;= limit</code>.</p>

<p>Return <em>the minimum number of elements you need to add to make the sum of the array equal to </em><code>goal</code>. The array must maintain its property that <code>abs(nums[i]) &lt;= limit</code>.</p>

<p>Note that <code>abs(x)</code> equals <code>x</code> if <code>x &gt;= 0</code>, and <code>-x</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,-1,1], limit = 3, goal = -4
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can add -2 and -3, then the sum of the array will be 1 - 1 + 1 - 2 - 3 = -4.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,-10,9,1], limit = 100, goal = 0
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= limit &lt;= 10<sup>6</sup></code></li>
	<li><code>-limit &lt;= nums[i] &lt;= limit</code></li>
	<li><code>-10<sup>9</sup> &lt;= goal &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Try thinking about the problem as if the array is empty. Then you only need to form goal using elements whose absolute value is <= limit.
2. You can greedily set all of the elements except one to limit or -limit, so the number of elements you need is ceil(abs(goal)/ limit).
3. You can "normalize" goal by offsetting it by the sum of the array. For example, if the goal is 5 and the sum is -3, then it's exactly the same as if the goal is 8 and the array is empty.
4. The answer is ceil(abs(goal-sum)/limit) = (abs(goal-sum)+limit-1) / limit.

## Solution

```rust
impl Solution { pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 { (((goal as i64 - nums.iter().map(|&x| x as i64).sum::<i64>()).abs() + limit as i64 - 1) / limit as i64) as i32 } }
```