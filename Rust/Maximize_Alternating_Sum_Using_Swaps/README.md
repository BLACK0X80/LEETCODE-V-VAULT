# Maximize Alternating Sum Using Swaps

**Difficulty:** Hard
**Tags:** Array, Greedy, Union-Find, Sorting

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>You want to maximize the <strong>alternating sum</strong> of <code>nums</code>, which is defined as the value obtained by <strong>adding</strong> elements at even indices and <strong>subtracting</strong> elements at odd indices. That is, <code>nums[0] - nums[1] + nums[2] - nums[3]...</code></p>

<p>You are also given a 2D integer array <code>swaps</code> where <code>swaps[i] = [p<sub>i</sub>, q<sub>i</sub>]</code>. For each pair <code>[p<sub>i</sub>, q<sub>i</sub>]</code> in <code>swaps</code>, you are allowed to swap the elements at indices <code>p<sub>i</sub></code> and <code>q<sub>i</sub></code>. These swaps can be performed any number of times and in any order.</p>

<p>Return the maximum possible <strong>alternating sum</strong> of <code>nums</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], swaps = [[0,2],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The maximum alternating sum is achieved when <code>nums</code> is <code>[2, 1, 3]</code> or <code>[3, 1, 2]</code>. As an example, you can obtain <code>nums = [2, 1, 3]</code> as follows.</p>

<ul>
	<li>Swap <code>nums[0]</code> and <code>nums[2]</code>. <code>nums</code> is now <code>[3, 2, 1]</code>.</li>
	<li>Swap <code>nums[1]</code> and <code>nums[2]</code>. <code>nums</code> is now <code>[3, 1, 2]</code>.</li>
	<li>Swap <code>nums[0]</code> and <code>nums[2]</code>. <code>nums</code> is now <code>[2, 1, 3]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], swaps = [[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The maximum alternating sum is achieved by not performing any swaps.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1000000000,1,1000000000,1,1000000000], swaps = []</span></p>

<p><strong>Output:</strong> <span class="example-io">-2999999997</span></p>

<p><strong>Explanation:</strong></p>

<p>Since we cannot perform any swaps, the maximum alternating sum is achieved by not performing any swaps.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= swaps.length &lt;= 10<sup>5</sup></code></li>
	<li><code>swaps[i] = [p<sub>i</sub>, q<sub>i</sub>]</code></li>
	<li><code>0 &lt;= p<sub>i</sub> &lt; q<sub>i</sub> &lt;= nums.length - 1</code></li>
	<li><code>[p<sub>i</sub>, q<sub>i</sub>] != [p<sub>j</sub>, q<sub>j</sub>]</code></li>
</ul>


## Hints

1. Build connected components using a DSU (disjoint-set union).
2. Let <code>E</code> be the count of even indices inside that component. In each component, place the largest <code>E</code> values on the component's even indices.
3. The component's contribution to the alternating sum is <code>2 * sumTopE - sumAll</code>, where <code>sumTopE</code> is the sum of the largest <code>E</code> values and <code>sumAll</code> is the sum of all values in the component.

## Solution

```rust
impl Solution {
    pub fn max_alternating_sum(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i64 {
        let black3 = black1.len();
        let mut black4 = (0..black3).collect::<Vec<_>>();
        fn find(b: &mut Vec<usize>, i: usize) -> usize {
            if b[i] == i { i } else { b[i] = find(b, b[i]); b[i] }
        }
        for s in black2 {
            let r1 = find(&mut black4, s[0] as usize);
            let r2 = find(&mut black4, s[1] as usize);
            if r1 != r2 { black4[r1] = r2; }
        }
        let mut black5 = std::collections::HashMap::new();
        for i in 0..black3 {
            let r = find(&mut black4, i);
            let entry = black5.entry(r).or_insert((Vec::new(), Vec::new()));
            if i % 2 == 0 { entry.0.push(i); } else { entry.1.push(i); }
        }
        let mut black6 = 0i64;
        for (_, (evens, odds)) in black5 {
            let mut vals = Vec::new();
            for &idx in &evens { vals.push(black1[idx]); }
            for &idx in &odds { vals.push(black1[idx]); }
            vals.sort_unstable_by(|a, b| b.cmp(a));
            for i in 0..evens.len() { black6 += vals[i] as i64; }
            for i in evens.len()..vals.len() { black6 -= vals[i] as i64; }
        }
        black6
    }
}
```