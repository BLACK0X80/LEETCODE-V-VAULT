# Maximum Total Subarray Value II

**Difficulty:** Hard
**Tags:** Array, Greedy, Segment Tree, Heap (Priority Queue)

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an integer <code>k</code>.</p>

<p>You must select <strong>exactly</strong> <code>k</code> <strong>distinct</strong> non-empty <span data-keyword="subarray-nonempty">subarrays</span> <code>nums[l..r]</code> of <code>nums</code>. Subarrays may overlap, but the exact same subarray (same <code>l</code> and <code>r</code>) <strong>cannot</strong> be chosen more than once.</p>

<p>The <strong>value</strong> of a subarray <code>nums[l..r]</code> is defined as: <code>max(nums[l..r]) - min(nums[l..r])</code>.</p>

<p>The <strong>total value</strong> is the sum of the <strong>values</strong> of all chosen subarrays.</p>

<p>Return the <strong>maximum</strong> possible total value you can achieve.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,2], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal approach is:</p>

<ul>
	<li>Choose <code>nums[0..1] = [1, 3]</code>. The maximum is 3 and the minimum is 1, giving a value of <code>3 - 1 = 2</code>.</li>
	<li>Choose <code>nums[0..2] = [1, 3, 2]</code>. The maximum is still 3 and the minimum is still 1, so the value is also <code>3 - 1 = 2</code>.</li>
</ul>

<p>Adding these gives <code>2 + 2 = 4</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,2,5,1], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal approach is:</p>

<ul>
	<li>Choose <code>nums[0..3] = [4, 2, 5, 1]</code>. The maximum is 5 and the minimum is 1, giving a value of <code>5 - 1 = 4</code>.</li>
	<li>Choose <code>nums[1..3] = [2, 5, 1]</code>. The maximum is 5 and the minimum is 1, so the value is also <code>4</code>.</li>
	<li>Choose <code>nums[2..3] = [5, 1]</code>. The maximum is 5 and the minimum is 1, so the value is again <code>4</code>.</li>
</ul>

<p>Adding these gives <code>4 + 4 + 4 = 12</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 5 * 10<sup>​​​​​​​4</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= min(10<sup>5</sup>, n * (n + 1) / 2)</code></li>
</ul>


## Hints

1. For fixed <code>l</code>, the sequence <code>v(l,r)=max(nums[l..r])−min(nums[l..r])</code> is non-increasing as <code>r</code> moves left.
2. Build RMQs (sparse tables) for range max/min so each <code>v(l,r)</code> is queryable in <code>O(1)</code>.
3. Use a max-heap with <code>v(l,n-1)</code> for all <code>l</code>; pop the largest <code>k</code> times, and after popping an entry from <code>(l,r)</code> push <code>(l,r-1)</code> if <code>r>l</code>.

## Solution

```rust
use std::collections::BinaryHeap;

struct BlackTable {
    black1: Vec<Vec<i32>>,
    black2: Vec<Vec<i32>>,
    black3: Vec<usize>,
}

impl BlackTable {
    fn new(black4: &[i32]) -> Self {
        let black5 = black4.len();
        let black6 = (black5 as f64).log2() as usize + 1;
        let mut black7 = vec![vec![0; black6]; black5];
        let mut black8 = vec![vec![0; black6]; black5];
        let mut black9 = vec![0; black5 + 1];

        for i in 2..=black5 { black9[i] = black9[i / 2] + 1; }
        for i in 0..black5 {
            black7[i][0] = black4[i];
            black8[i][0] = black4[i];
        }

        for j in 1..black6 {
            for i in 0..=(black5 - (1 << j)) {
                black7[i][j] = black7[i][j - 1].min(black7[i + (1 << (j - 1))][j - 1]);
                black8[i][j] = black8[i][j - 1].max(black8[i + (1 << (j - 1))][j - 1]);
            }
        }
        Self { black1: black7, black2: black8, black3: black9 }
    }

    fn black_q(&self, l: usize, r: usize) -> i32 {
        let j = self.black3[r - l + 1];
        let mn = self.black1[l][j].min(self.black1[r - (1 << j) + 1][j]);
        let mx = self.black2[l][j].max(self.black2[r - (1 << j) + 1][j]);
        mx - mn
    }
}

impl Solution {
    pub fn max_total_value(black10: Vec<i32>, mut black11: i32) -> i64 {
        let black12 = black10.len();
        let black13 = BlackTable::new(&black10);
        let mut black14 = BinaryHeap::new();
        let mut black15 = 0i64;

        for i in 0..black12 {
            black14.push((black13.black_q(0, i), 0, i));
        }

        while black11 > 0 && !black14.is_empty() {
            let (black16, l, r) = black14.pop().unwrap();
            black15 += black16 as i64;
            if l + 1 <= r {
                black14.push((black13.black_q(l + 1, r), l + 1, r));
            }
            black11 -= 1;
        }
        black15
    }
}
```