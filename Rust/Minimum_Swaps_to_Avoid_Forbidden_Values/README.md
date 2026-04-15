# Minimum Swaps to Avoid Forbidden Values

**Difficulty:** Hard
**Tags:** Array, Hash Table, Greedy, Counting

---

## Problem

<p>You are given two integer arrays, <code>nums</code> and <code>forbidden</code>, each of length <code>n</code>.</p>

<p>You may perform the following operation any number of times (including zero):</p>

<ul>
	<li>Choose two <strong>distinct</strong> indices <code>i</code> and <code>j</code>, and swap <code>nums[i]</code> with <code>nums[j]</code>.</li>
</ul>

<p>Return the <strong>minimum</strong> number of swaps required such that, for every index <code>i</code>, the value of <code>nums[i]</code> is <strong>not equal</strong> to <code>forbidden[i]</code>. If no amount of swaps can ensure that every index avoids its forbidden value, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], forbidden = [3,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal set of swaps:</p>

<ul>
	<li>Select indices <code>i = 0</code> and <code>j = 1</code> in <code>nums</code> and swap them, resulting in <code>nums = [2, 1, 3]</code>.</li>
	<li>After this swap, for every index <code>i</code>, <code>nums[i]</code> is not equal to <code>forbidden[i]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,6,6,5], forbidden = [4,6,5,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>
One optimal set of swaps:

<ul>
	<li>Select indices <code>i = 0</code> and <code>j = 2</code> in <code>nums</code> and swap them, resulting in <code>nums = [6, 6, 4, 5]</code>.</li>
	<li>Select indices <code>i = 1</code> and <code>j = 3</code> in <code>nums</code> and swap them, resulting in <code>nums = [6, 5, 4, 6]</code>.</li>
	<li>After these swaps, for every index <code>i</code>, <code>nums[i]</code> is not equal to <code>forbidden[i]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [7,7], forbidden = [8,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>
It is not possible to make <code>nums[i]</code> different from <code>forbidden[i]</code> for all indices.</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2], forbidden = [2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>No swaps are required because <code>nums[i]</code> is already different from <code>forbidden[i]</code> for all indices, so the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length == forbidden.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], forbidden[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Solve the problem greedily.
2. Count combined frequencies of values in <code>nums</code> and <code>forbidden</code> into a map <code>freq</code>.
3. If any <code>freq[val] >= n + 1</code> return <code>-1</code> (impossible).
4. Collect bad positions (<code>nums[i] == forbidden[i]</code>) into a map <code>badPairs[val]</code> counts.
5. Let <code>badPairsSum</code> be the sum of all bad counts and <code>maxBadPairs</code> the maximum bad count for any single value.
6. The minimum swaps equals <code>max((badPairsSum + 1) / 2, maxBadPairs)</code> (i.e. ceil(badPairsSum/2) vs the largest same-value bad cluster).

## Solution

```rust
impl Solution {
    pub fn min_swaps(black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let black3 = black1.len();
        let mut black4 = Vec::new();
        let mut black5 = std::collections::HashMap::new();
        for i in 0..black3 {
            if black1[i] == black2[i] {
                black4.push(i);
                *black5.entry(black1[i]).or_insert(0) += 1;
            }
        }
        if black4.is_empty() { return 0; }
        let (mut black6, mut black7) = (0, 0);
        for (&v, &c) in &black5 {
            if c > black6 { black6 = c; black7 = v; }
        }
        let black8 = black4.len();
        if 2 * black6 <= black8 { return ((black8 + 1) / 2) as i32; }
        let mut black9 = 2 * black6 - black8;
        let mut black10 = 0;
        for i in 0..black3 {
            if black1[i] != black2[i] && black1[i] != black7 && black2[i] != black7 { black10 += 1; }
        }
        if black10 < black9 { -1 } else { black6 as i32 }
    }
}
```