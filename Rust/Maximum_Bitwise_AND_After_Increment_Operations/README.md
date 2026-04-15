# Maximum Bitwise AND After Increment Operations

**Difficulty:** Hard
**Tags:** Array, Greedy, Bit Manipulation, Sorting

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>k</code> and <code>m</code>.</p>

<p>You may perform <strong>at most</strong> <code>k</code> operations. In one operation, you may choose any index <code>i</code> and <strong>increase</strong> <code>nums[i]</code> by 1.</p>

<p>Return an integer denoting the <strong>maximum</strong> possible <strong>bitwise AND</strong> of any <strong><span data-keyword="subset">subset</span></strong> of size <code>m</code> after performing up to <code>k</code> operations optimally.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2], k = 8, m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>We need a subset of size <code>m = 2</code>. Choose indices <code>[0, 2]</code>.</li>
	<li>Increase <code>nums[0] = 3</code> to 6 using 3 operations, and increase <code>nums[2] = 2</code> to 6 using 4 operations.</li>
	<li>The total number of operations used is 7, which is not greater than <code>k = 8</code>.</li>
	<li>The two chosen values become <code>[6, 6]</code>, and their bitwise AND is <code>6</code>, which is the maximum possible.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,8,4], k = 7, m = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>We need a subset of size <code>m = 3</code>. Choose indices <code>[0, 1, 3]</code>.</li>
	<li>Increase <code>nums[0] = 1</code> to 4 using 3 operations, increase <code>nums[1] = 2</code> to 4 using 2 operations, and keep <code>nums[3] = 4</code>.</li>
	<li>The total number of operations used is 5, which is not greater than <code>k = 7</code>.</li>
	<li>The three chosen values become <code>[4, 4, 4]</code>, and their bitwise AND is 4, which is the maximum possible.​​​​​​​</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1], k = 3, m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>We need a subset of size <code>m = 2</code>. Choose indices <code>[0, 1]</code>.</li>
	<li>Increase both values from 1 to 2 using 1 operation each.</li>
	<li>The total number of operations used is 2, which is not greater than <code>k = 3</code>.</li>
	<li>The two chosen values become <code>[2, 2]</code>, and their bitwise AND is 2, which is the maximum possible.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= m &lt;= n</code></li>
</ul>


## Hints

1. Use a greedy bitwise approach.
2. Iterate bits from highest to lowest and try setting the current bit in a candidate <code>res</code>.
3. To test a candidate, for each <code>num</code> compute the minimal increments needed so that <code>(num | candidate) == candidate</code>; take the smallest <code>m</code> costs and check if their sum <= <code>k</code>.
4. If feasible, keep the bit in <code>res</code> and continue with accumulated bits.

## Solution

```rust
impl Solution {
    pub fn maximum_and(black1: Vec<i32>, black2: i32, black3: i32) -> i32 {
        let mut black4 = 0;
        let black5 = black2 as i64;
        let black6 = black3 as usize;
        for i in (0..31).rev() {
            let black7 = black4 | (1 << i);
            let mut black8 = Vec::with_capacity(black1.len());
            for &black9 in &black1 {
                if (black9 & black7) == black7 {
                    black8.push(0i64);
                } else {
                    let mut black10 = 0i64;
                    for j in (0..31).rev() {
                        if ((black7 >> j) & 1 == 1) && ((black9 >> j) & 1 == 0) {
                            black10 = (((black9 as i64 >> j) + 1) << j | black7 as i64) - black9 as i64;
                            break;
                        }
                    }
                    black8.push(black10);
                }
            }
            black8.sort_unstable();
            let mut black11 = 0i64;
            for j in 0..black6 { black11 += black8[j]; }
            if black11 <= black5 { black4 = black7; }
        }
        black4
    }
}
```