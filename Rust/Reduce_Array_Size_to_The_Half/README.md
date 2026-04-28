# Reduce Array Size to The Half

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given an integer array <code>arr</code>. You can choose a set of integers and remove all the occurrences of these integers in the array.</p>

<p>Return <em>the minimum size of the set so that <strong>at least</strong> half of the integers of the array are removed</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,3,3,3,5,5,5,2,2,7]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of the size of the old array).
Possible sets of size 2 are {3,5},{3,2},{5,2}.
Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has a size greater than half of the size of the old array.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [7,7,7,7,7,7]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible set you can choose is {7}. This will make the new array empty.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>arr.length</code> is even.</li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Count the frequency of each integer in the array.
2. Start with an empty set, add to the set the integer with the maximum frequency.
3. Keep Adding the integer with the max frequency until you remove at least half of the integers.

## Solution

```rust
impl Solution { pub fn min_set_size(black_arr: Vec<i32>) -> i32 { let mut black_f = std::collections::HashMap::new(); for black_x in &black_arr { *black_f.entry(black_x).or_insert(0) += 1; } let mut black_v: Vec<i32> = black_f.into_values().collect(); black_v.sort_unstable_by(|black_a, black_b| black_b.cmp(black_a)); let (mut black_s, mut black_c, black_h) = (0, 0, (black_arr.len() / 2) as i32); for black_v in black_v { black_s += black_v; black_c += 1; if black_s >= black_h { break; } } black_c } }
```