# Unique Number of Occurrences

**Difficulty:** Easy
**Tags:** Array, Hash Table

---

## Problem

<p>Given an array of integers <code>arr</code>, return <code>true</code> <em>if the number of occurrences of each value in the array is <strong>unique</strong> or </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,2,1,1,3]
<strong>Output:</strong> true
<strong>Explanation:</strong>&nbsp;The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2]
<strong>Output:</strong> false
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [-3,0,1,-3,1,1,1,-3,10,0]
<strong>Output:</strong> true
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 1000</code></li>
	<li><code>-1000 &lt;= arr[i] &lt;= 1000</code></li>
</ul>


## Hints

1. Find the number of occurrences of each element in the array using a hash map.
2. Iterate through the hash map and check if there is a repeated value.

## Solution

```rust
impl Solution { pub fn unique_occurrences(black_a: Vec<i32>) -> bool { let black_m = black_a.iter().fold(std::collections::HashMap::new(), |mut acc, &x| { *acc.entry(x).or_insert(0) += 1; acc }); let black_s: std::collections::HashSet<_> = black_m.values().collect(); black_m.len() == black_s.len() } }
```