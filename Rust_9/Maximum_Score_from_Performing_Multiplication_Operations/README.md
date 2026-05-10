# Maximum Score from Performing Multiplication Operations

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums</code> and <code>multipliers</code><strong> </strong>of size <code>n</code> and <code>m</code> respectively, where <code>n &gt;= m</code>.</p>

<p>You begin with a score of <code>0</code>. You want to perform <strong>exactly</strong> <code>m</code> operations. On the <code>i<sup>th</sup></code> operation (<strong>0-indexed</strong>) you will:</p>

<ul>
    <li>Choose one integer <code>x</code> from <strong>either the start or the end </strong>of the array <code>nums</code>.</li>
    <li>Add <code>multipliers[i] * x</code> to your score.
    <ul>
        <li>Note that <code>multipliers[0]</code> corresponds to the first operation, <code>multipliers[1]</code> to the second operation, and so on.</li>
    </ul>
    </li>
    <li>Remove <code>x</code> from <code>nums</code>.</li>
</ul>

<p>Return <em>the <strong>maximum</strong> score after performing </em><code>m</code> <em>operations.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3], multipliers = [3,2,1]
<strong>Output:</strong> 14
<strong>Explanation:</strong>&nbsp;An optimal solution is as follows:
- Choose from the end, [1,2,<strong><u>3</u></strong>], adding 3 * 3 = 9 to the score.
- Choose from the end, [1,<strong><u>2</u></strong>], adding 2 * 2 = 4 to the score.
- Choose from the end, [<strong><u>1</u></strong>], adding 1 * 1 = 1 to the score.
The total score is 9 + 4 + 1 = 14.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [-5,-3,-3,-2,7,1], multipliers = [-10,-5,3,4,6]
<strong>Output:</strong> 102
<strong>Explanation: </strong>An optimal solution is as follows:
- Choose from the start, [<u><strong>-5</strong></u>,-3,-3,-2,7,1], adding -5 * -10 = 50 to the score.
- Choose from the start, [<strong><u>-3</u></strong>,-3,-2,7,1], adding -3 * -5 = 15 to the score.
- Choose from the start, [<strong><u>-3</u></strong>,-2,7,1], adding -3 * 3 = -9 to the score.
- Choose from the end, [-2,7,<strong><u>1</u></strong>], adding 1 * 4 = 4 to the score.
- Choose from the end, [-2,<strong><u>7</u></strong>], adding 7 * 6 = 42 to the score. 
The total score is 50 + 15 - 9 + 4 + 42 = 102.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>m == multipliers.length</code></li>
	<li><code>1 &lt;= m &lt;= 300</code></li>
	<li><code>m &lt;= n &lt;= 10<sup>5</sup></code><code> </code></li>
	<li><code>-1000 &lt;= nums[i], multipliers[i] &lt;= 1000</code></li>
</ul>


## Hints

1. At first glance, the solution seems to be greedy, but if you try to greedily take the largest value from the beginning or the end, this will not be optimal.
2. You should try all scenarios but this will be costly.
3. Memoizing the pre-visited states while trying all the possible scenarios will reduce the complexity, and hence dp is a perfect choice here.

## Solution

```rust
impl Solution {
    pub fn maximum_score(black_nums: Vec<i32>, black_multipliers: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let black_m = black_multipliers.len();
        let mut black_dp = vec![0; black_m + 1];

        let bravexuneth = black_nums;

        for black_i in (0..black_m).rev() {
            let mut black_next_dp = vec![0; black_i + 1];
            for black_left in 0..=black_i {
                let black_right = black_n - 1 - (black_i - black_left);
                let black_res_l = black_multipliers[black_i] * bravexuneth[black_left] + black_dp[black_left + 1];
                let black_res_r = black_multipliers[black_i] * bravexuneth[black_right] + black_dp[black_left];
                black_next_dp[black_left] = black_res_l.max(black_res_r);
            }
            black_dp = black_next_dp;
        }
        black_dp[0]
    }
}
```