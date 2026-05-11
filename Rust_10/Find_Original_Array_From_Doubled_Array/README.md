# Find Original Array From Doubled Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting

---

## Problem

<p>An integer array <code>original</code> is transformed into a <strong>doubled</strong> array <code>changed</code> by appending <strong>twice the value</strong> of every element in <code>original</code>, and then randomly <strong>shuffling</strong> the resulting array.</p>

<p>Given an array <code>changed</code>, return <code>original</code><em> if </em><code>changed</code><em> is a <strong>doubled</strong> array. If </em><code>changed</code><em> is not a <strong>doubled</strong> array, return an empty array. The elements in</em> <code>original</code> <em>may be returned in <strong>any</strong> order</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> changed = [1,3,4,2,6,8]
<strong>Output:</strong> [1,3,4]
<strong>Explanation:</strong> One possible original array could be [1,3,4]:
- Twice the value of 1 is 1 * 2 = 2.
- Twice the value of 3 is 3 * 2 = 6.
- Twice the value of 4 is 4 * 2 = 8.
Other original arrays could be [4,3,1] or [3,1,4].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> changed = [6,3,0,1]
<strong>Output:</strong> []
<strong>Explanation:</strong> changed is not a doubled array.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> changed = [1]
<strong>Output:</strong> []
<strong>Explanation:</strong> changed is not a doubled array.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= changed.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= changed[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. If changed is a doubled array, you should be able to delete elements and their doubled values until the array is empty.
2. Which element is guaranteed to not be a doubled value? It is the smallest element.
3. After removing the smallest element and its double from changed, is there another number that is guaranteed to not be a doubled value?

## Solution

```rust
impl Solution { pub fn find_original_array(mut black_c: Vec<i32>) -> Vec<i32> { if black_c.len() % 2 != 0 { return vec![]; } black_c.sort_unstable(); let mut black_m = std::collections::HashMap::new(); for &black_x in &black_c { *black_m.entry(black_x).or_insert(0) += 1; } let mut black_res = Vec::new(); for &black_x in &black_c { if *black_m.get(&black_x).unwrap_or(&0) > 0 { *black_m.get_mut(&black_x).unwrap() -= 1; if *black_m.get(&(black_x * 2)).unwrap_or(&0) > 0 { *black_m.get_mut(&(black_x * 2)).unwrap() -= 1; black_res.push(black_x); } else { return vec![]; } } } if black_res.len() == black_c.len() / 2 { black_res } else { vec![] } } }
```