# Find Array Given Subset Sums

**Difficulty:** Hard
**Tags:** Array, Hash Table, Sorting, Counting

---

## Problem

<p>You are given an integer <code>n</code> representing the length of an unknown array that you are trying to recover. You are also given an array <code>sums</code> containing the values of all <code>2<sup>n</sup></code> <strong>subset sums</strong> of the unknown array (in no particular order).</p>

<p>Return <em>the array </em><code>ans</code><em> of length </em><code>n</code><em> representing the unknown array. If <strong>multiple</strong> answers exist, return <strong>any</strong> of them</em>.</p>

<p>An array <code>sub</code> is a <strong>subset</strong> of an array <code>arr</code> if <code>sub</code> can be obtained from <code>arr</code> by deleting some (possibly zero or all) elements of <code>arr</code>. The sum of the elements in <code>sub</code> is one possible <strong>subset sum</strong> of <code>arr</code>. The sum of an empty array is considered to be <code>0</code>.</p>

<p><strong>Note:</strong> Test cases are generated such that there will <strong>always</strong> be at least one correct answer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3, sums = [-3,-2,-1,0,0,1,2,3]
<strong>Output:</strong> [1,2,-3]
<strong>Explanation: </strong>[1,2,-3] is able to achieve the given subset sums:
- []: sum is 0
- [1]: sum is 1
- [2]: sum is 2
- [1,2]: sum is 3
- [-3]: sum is -3
- [1,-3]: sum is -2
- [2,-3]: sum is -1
- [1,2,-3]: sum is 0
Note that any permutation of [1,2,-3] and also any permutation of [-1,-2,3] will also be accepted.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2, sums = [0,0,0,0]
<strong>Output:</strong> [0,0]
<strong>Explanation:</strong> The only correct answer is [0,0].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 4, sums = [0,0,5,5,4,-1,4,9,9,-1,4,3,4,8,3,8]
<strong>Output:</strong> [0,-1,4,5]
<strong>Explanation:</strong> [0,-1,4,5] is able to achieve the given subset sums.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 15</code></li>
	<li><code>sums.length == 2<sup>n</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= sums[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. What information do the two largest elements tell us?
2. Can we use recursion to check all possible states?

## Solution

```rust
impl Solution {
    pub fn recover_array(n: i32, mut sums: Vec<i32>) -> Vec<i32> {
        sums.sort_unstable();
        let mut black = (sums, vec![], n as usize);
        while black.2 > 0 {
            let black_diff = black.0[1] - black.0[0];
            let (mut black_s0, mut black_s1) = (vec![], vec![]);
            let mut black_map = std::collections::HashMap::new();
            let mut black_found_zero = false;
            for &black_v in &black.0 {
                if *black_map.get(&(black_v - black_diff)).unwrap_or(&0) > 0 {
                    black_s1.push(black_v);
                    *black_map.get_mut(&(black_v - black_diff)).unwrap_or(&mut 0) -= 1;
                } else {
                    black_s0.push(black_v);
                    *black_map.entry(black_v).or_insert(0) += 1;
                }
            }
            for &black_v in &black_s0 { if black_v == 0 { black_found_zero = true; break; } }
            if black_found_zero {
                black.1.push(black_diff);
                black.0 = black_s0;
            } else {
                black.1.push(-black_diff);
                black.0 = black_s1;
            }
            black.2 -= 1;
        }
        black.1
    }
}
```