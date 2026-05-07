# Distribute Repeating Integers

**Difficulty:** Hard
**Tags:** Array, Hash Table, Dynamic Programming, Backtracking, Bit Manipulation, Counting, Bitmask

---

## Problem

<p>You are given an array of <code>n</code> integers, <code>nums</code>, where there are at most <code>50</code> unique values in the array. You are also given an array of <code>m</code> customer order quantities, <code>quantity</code>, where <code>quantity[i]</code> is the amount of integers the <code>i<sup>th</sup></code> customer ordered. Determine if it is possible to distribute <code>nums</code> such that:</p>

<ul>
	<li>The <code>i<sup>th</sup></code> customer gets <strong>exactly</strong> <code>quantity[i]</code> integers,</li>
	<li>The integers the <code>i<sup>th</sup></code> customer gets are <strong>all equal</strong>, and</li>
	<li>Every customer is satisfied.</li>
</ul>

<p>Return <code>true</code><em> if it is possible to distribute </em><code>nums</code><em> according to the above conditions</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], quantity = [2]
<strong>Output:</strong> false
<strong>Explanation:</strong> The 0<sup>th</sup> customer cannot be given two different integers.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,3], quantity = [2]
<strong>Output:</strong> true
<strong>Explanation:</strong> The 0<sup>th</sup> customer is given [3,3]. The integers [1,2] are not used.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,2,2], quantity = [2,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> The 0<sup>th</sup> customer is given [1,1], and the 1st customer is given [2,2].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 1000</code></li>
	<li><code>m == quantity.length</code></li>
	<li><code>1 &lt;= m &lt;= 10</code></li>
	<li><code>1 &lt;= quantity[i] &lt;= 10<sup>5</sup></code></li>
	<li>There are at most <code>50</code> unique values in <code>nums</code>.</li>
</ul>


## Hints

1. Count the frequencies of each number. For example, if nums = [4,4,5,5,5], frequencies = [2,3].
2. Each customer wants all of their numbers to be the same. This means that each customer will be assigned to one number.
3. Use dynamic programming. Iterate through the numbers' frequencies, and choose some subset of customers to be assigned to this number.

## Solution

```rust
impl Solution {
    pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
        let mut cnt = [0i32; 1001];
        for x in nums { cnt[x as usize] += 1; }
        let mut b: Vec<i32> = cnt.into_iter().filter(|&c| c > 0).collect();
        b.sort_unstable_by(|a, b| b.cmp(a));
        quantity.sort_unstable_by(|a, b| b.cmp(a));

        fn dfs(b: &mut Vec<i32>, q: &[i32], i: usize) -> bool {
            if i == q.len() { return true; }
            let v = q[i];
            for j in 0..b.len() {
                if b[j] >= v {
                    b[j] -= v;
                    if dfs(b, q, i + 1) { return true; }
                    b[j] += v;
                    if b[j] == v { return false; }
                }
            }
            false
        }

        dfs(&mut b, &quantity, 0)
    }
}
```