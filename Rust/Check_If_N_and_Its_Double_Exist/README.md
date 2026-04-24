# Check If N and Its Double Exist

**Difficulty:** Easy
**Tags:** Array, Hash Table, Two Pointers, Binary Search, Sorting

---

## Problem

<p>Given an array <code>arr</code> of integers, check if there exist two indices <code>i</code> and <code>j</code> such that :</p>

<ul>
	<li><code>i != j</code></li>
	<li><code>0 &lt;= i, j &lt; arr.length</code></li>
	<li><code>arr[i] == 2 * arr[j]</code></li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [10,2,5,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,1,7,11]
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no i and j that satisfy the conditions.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= arr.length &lt;= 500</code></li>
	<li><code>-10<sup>3</sup> &lt;= arr[i] &lt;= 10<sup>3</sup></code></li>
</ul>


## Hints

1. Loop from i = 0 to arr.length, maintaining in a hashTable the array elements from [0, i - 1].
2. On each step of the loop check if we have seen the element <code>2 * arr[i]</code> so far.
3. Also check if we have seen <code>arr[i] / 2</code> in case <code>arr[i] % 2 == 0</code>.

## Solution

```rust
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut black = std::collections::HashSet::new();
        arr.iter().any(|&noir| { let dark = black.contains(&(noir * 2)) || (noir % 2 == 0 && black.contains(&(noir / 2))); black.insert(noir); dark })
    }
}
```