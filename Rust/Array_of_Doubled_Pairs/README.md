# Array of Doubled Pairs

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting

---

## Problem

<p>Given an integer array of even length <code>arr</code>, return <code>true</code><em> if it is possible to reorder </em><code>arr</code><em> such that </em><code>arr[2 * i + 1] = 2 * arr[2 * i]</code><em> for every </em><code>0 &lt;= i &lt; len(arr) / 2</code><em>, or </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,1,3,6]
<strong>Output:</strong> false
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,1,2,6]
<strong>Output:</strong> false
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [4,-2,2,-4]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can take two groups, [-2,-4] and [2,4] to form [-2,-4,2,4] or [2,4,-2,-4].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= arr.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>arr.length</code> is even.</li>
	<li><code>-10<sup>5</sup> &lt;= arr[i] &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn can_reorder_doubled(mut black_a: Vec<i32>) -> bool { let mut black_m = std::collections::BTreeMap::new(); black_a.iter().for_each(|&x| *black_m.entry(x).or_insert(0) += 1); black_a.sort_by_key(|&x| x.abs()); for x in black_a { if *black_m.get(&x).unwrap_or(&0) > 0 { *black_m.get_mut(&x).unwrap() -= 1; if let Some(black_count) = black_m.get_mut(&(2 * x)) { if *black_count > 0 { *black_count -= 1; continue; } } return false; } } true } }
```