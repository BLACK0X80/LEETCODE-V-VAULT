# Pizza With 3n Slices

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Greedy, Heap (Priority Queue)

---

## Problem

<p>There is a pizza with <code>3n</code> slices of varying size, you and your friends will take slices of pizza as follows:</p>

<ul>
	<li>You will pick <strong>any</strong> pizza slice.</li>
	<li>Your friend Alice will pick the next slice in the anti-clockwise direction of your pick.</li>
	<li>Your friend Bob will pick the next slice in the clockwise direction of your pick.</li>
	<li>Repeat until there are no more slices of pizzas.</li>
</ul>

<p>Given an integer array <code>slices</code> that represent the sizes of the pizza slices in a clockwise direction, return <em>the maximum possible sum of slice sizes that you can pick</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/02/18/sample_3_1723.png" style="width: 500px; height: 266px;" />
<pre>
<strong>Input:</strong> slices = [1,2,3,4,5,6]
<strong>Output:</strong> 10
<strong>Explanation:</strong> Pick pizza slice of size 4, Alice and Bob will pick slices with size 3 and 5 respectively. Then Pick slices with size 6, finally Alice and Bob will pick slice of size 2 and 1 respectively. Total = 4 + 6.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/02/18/sample_4_1723.png" style="width: 500px; height: 299px;" />
<pre>
<strong>Input:</strong> slices = [8,9,8,6,1,1]
<strong>Output:</strong> 16
<strong>Explanation:</strong> Pick pizza slice of size 8 in each turn. If you pick slice with size 9 your partners will pick slices of size 8.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 * n == slices.length</code></li>
	<li><code>1 &lt;= slices.length &lt;= 500</code></li>
	<li><code>1 &lt;= slices[i] &lt;= 1000</code></li>
</ul>


## Hints

1. By studying the pattern of the operations, we can find out that the problem is equivalent to: Given an integer array with size 3N, select N integers with maximum sum and any selected integers are not next to each other in the array.
2. The first one in the array is considered next to the last one in the array. Use Dynamic Programming to solve it.

## Solution

```rust
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let black_m = slices.len();
        let black_n = black_m / 3;

        fn black_solve(black_arr: &[i32], black_count: usize) -> i32 {
            let black_len = black_arr.len();
            let mut black_dp = vec![vec![0; black_count + 1]; black_len + 1];

            for black_i in 1..=black_len {
                for black_j in 1..=black_count {
                    if black_i == 1 {
                        black_dp[black_i][black_j] = black_arr[0];
                    } else {
                        black_dp[black_i][black_j] = black_dp[black_i - 1][black_j].max(
                            black_dp[black_i - 2][black_j - 1] + black_arr[black_i - 1]
                        );
                    }
                }
            }
            black_dp[black_len][black_count]
        }

        let black_case1 = black_solve(&slices[0..black_m - 1], black_n);
        let black_case2 = black_solve(&slices[1..black_m], black_n);

        black_case1.max(black_case2)
    }
}
```