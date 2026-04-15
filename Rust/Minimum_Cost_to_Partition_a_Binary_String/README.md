# Minimum Cost to Partition a Binary String

**Difficulty:** Hard
**Tags:** String, Divide and Conquer, Prefix Sum

---

## Problem

<p>You are given a binary string <code>s</code> and two integers <code>encCost</code> and <code>flatCost</code>.</p>

<p>For each index <code>i</code>, <code>s[i] = &#39;1&#39;</code> indicates that the <code>i<sup>th</sup></code> element is sensitive, and <code>s[i] = &#39;0&#39;</code> indicates that it is not.</p>

<p>The string must be partitioned into <strong>segments</strong>. Initially, the entire string forms a single segment.</p>

<p>For a segment of length <code>L</code> containing <code>X</code> sensitive elements:</p>

<ul>
	<li>If <code>X = 0</code>, the cost is <code>flatCost</code>.</li>
	<li>If <code>X &gt; 0</code>, the cost is <code>L * X * encCost</code>.</li>
</ul>

<p>If a segment has <strong>even length</strong>, you may split it into <strong>two contiguous segments</strong> of <strong>equal</strong> length and the cost of this split is the <strong>sum</strong> of <strong>costs</strong> of the resulting segments.</p>

<p>Return an integer denoting the <strong>minimum possible total cost</strong> over all valid partitions.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1010&quot;, encCost = 2, flatCost = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The entire string <code>s = &quot;1010&quot;</code> has length 4 and contains 2 sensitive elements, giving a cost of <code>4 * 2 * 2 = 16</code>.</li>
	<li>Since the length is even, it can be split into <code>&quot;10&quot;</code> and <code>&quot;10&quot;</code>. Each segment has length 2 and contains 1 sensitive element, so each costs <code>2 * 1 * 2 = 4</code>, giving a total of 8.</li>
	<li>Splitting both segments into four single-character segments yields the segments <code>&quot;1&quot;</code>, <code>&quot;0&quot;</code>, <code>&quot;1&quot;</code>, and <code>&quot;0&quot;</code>. A segment containing <code>&quot;1&quot;</code> has length 1 and exactly one sensitive element, giving a cost of <code>1 * 1 * 2 = 2</code>, while a segment containing <code>&quot;0&quot;</code> has no sensitive elements and therefore costs <code>flatCost = 1</code>.</li>
	<li>​​​​​​​The total cost is thus <code>2 + 1 + 2 + 1 = 6</code>, which is the minimum possible total cost.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1010&quot;, encCost = 3, flatCost = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The entire string <code>s = &quot;1010&quot;</code> has length 4 and contains 2 sensitive elements, giving a cost of <code>4 * 2 * 3 = 24</code>.</li>
	<li>Since the length is even, it can be split into two segments <code>&quot;10&quot;</code> and <code>&quot;10&quot;</code>.</li>
	<li>Each segment has length 2 and contains one sensitive element, so each costs <code>2 * 1 * 3 = 6</code>, giving a total of 12, which is the minimum possible total cost.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;00&quot;, encCost = 1, flatCost = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The string <code>s = &quot;00&quot;</code> has length 2 and contains no sensitive elements, so storing it as a single segment costs <code>flatCost = 2</code>, which is the minimum possible total cost.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists only of <code>&#39;0&#39;</code> and <code>&#39;1&#39;</code>.</li>
	<li><code>1 &lt;= encCost, flatCost &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Use divide and conquer: recursively evaluate each segment; if its length is even you may split it into two equal halves.
2. For a segment, let <code>cost_no_split = (X == 0 ? flatCost : L * X * encCost)</code>. If length is even, compute <code>cost_split = cost(left) + cost(right)</code> and take the minimum.
3. Precompute prefix sums of ones so <code>X</code> (number of '1's) for any segment is O(1).
4. Memoize results for segments.

## Solution

```rust
impl Solution {
    pub fn min_cost(black_s: String, black_enc_cost: i32, black_flat_cost: i32) -> i64 {
        let mut black_psum = vec![0];
        for black_ch in black_s.chars() {
            black_psum.push(black_psum.last().unwrap() + (if black_ch == '1' { 1 } else { 0 }));
        }
        Self::solve(0, black_s.len(), black_enc_cost as i64, black_flat_cost as i64, &black_psum)
    }

    fn solve(black_l: usize, black_r: usize, black_enc: i64, black_flat: i64, black_psum: &Vec<i32>) -> i64 {
        let black_cnt = (black_psum[black_r] - black_psum[black_l]) as i64;
        let black_sz = (black_r - black_l) as i64;
        
        let mut black_cost = if black_cnt == 0 {
            black_flat
        } else {
            black_cnt * black_sz * black_enc
        };

        if black_cnt > 0 && black_sz % 2 == 0 {
            let black_mid = black_l + (black_r - black_l) / 2;
            let black_split_cost = Self::solve(black_l, black_mid, black_enc, black_flat, black_psum) +
                                  Self::solve(black_mid, black_r, black_enc, black_flat, black_psum);
            black_cost = black_cost.min(black_split_cost);
        }
        
        black_cost
    }
}
```