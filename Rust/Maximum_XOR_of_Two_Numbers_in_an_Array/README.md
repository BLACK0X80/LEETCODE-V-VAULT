# Maximum XOR of Two Numbers in an Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Bit Manipulation, Trie

---

## Problem

<p>Given an integer array <code>nums</code>, return <em>the maximum result of </em><code>nums[i] XOR nums[j]</code>, where <code>0 &lt;= i &lt;= j &lt; n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,10,5,25,2,8]
<strong>Output:</strong> 28
<strong>Explanation:</strong> The maximum result is 5 XOR 25 = 28.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [14,70,53,83,49,91,36,80,92,51,66,70]
<strong>Output:</strong> 127
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_maximum_xor(black_n: Vec<i32>) -> i32 { let (mut black_mask, mut black_res) = (0, 0); for black_i in (0..31).rev() { black_mask |= 1 << black_i; let black_prefixes: std::collections::HashSet<_> = black_n.iter().map(|&x| x & black_mask).collect(); let black_target = black_res | (1 << black_i); if black_prefixes.iter().any(|&p| black_prefixes.contains(&(p ^ black_target))) { black_res = black_target; } } black_res } }
```