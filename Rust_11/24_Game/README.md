# 24 Game

**Difficulty:** Hard
**Tags:** Array, Math, Backtracking

---

## Problem

<p>You are given an integer array <code>cards</code> of length <code>4</code>. You have four cards, each containing a number in the range <code>[1, 9]</code>. You should arrange the numbers on these cards in a mathematical expression using the operators <code>[&#39;+&#39;, &#39;-&#39;, &#39;*&#39;, &#39;/&#39;]</code> and the parentheses <code>&#39;(&#39;</code> and <code>&#39;)&#39;</code> to get the value 24.</p>

<p>You are restricted with the following rules:</p>

<ul>
	<li>The division operator <code>&#39;/&#39;</code> represents real division, not integer division.

	<ul>
		<li>For example, <code>4 / (1 - 2 / 3) = 4 / (1 / 3) = 12</code>.</li>
	</ul>
	</li>
	<li>Every operation done is between two numbers. In particular, we cannot use <code>&#39;-&#39;</code> as a unary operator.
	<ul>
		<li>For example, if <code>cards = [1, 1, 1, 1]</code>, the expression <code>&quot;-1 - 1 - 1 - 1&quot;</code> is <strong>not allowed</strong>.</li>
	</ul>
	</li>
	<li>You cannot concatenate numbers together
	<ul>
		<li>For example, if <code>cards = [1, 2, 1, 2]</code>, the expression <code>&quot;12 + 12&quot;</code> is not valid.</li>
	</ul>
	</li>
</ul>

<p>Return <code>true</code> if you can get such expression that evaluates to <code>24</code>, and <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> cards = [4,1,8,7]
<strong>Output:</strong> true
<strong>Explanation:</strong> (8-4) * (7-1) = 24
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> cards = [1,2,1,2]
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>cards.length == 4</code></li>
	<li><code>1 &lt;= cards[i] &lt;= 9</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn judge_point24(black_cards: Vec<i32>) -> bool {
        let mut black_nums: Vec<f64> = black_cards.into_iter().map(|black_x| black_x as f64).collect();
        Self::black_solve(&mut black_nums)
    }

    fn black_solve(black_nums: &mut Vec<f64>) -> bool {
        let black_n = black_nums.len();
        if black_n == 1 {
            return (black_nums[0] - 24.0).abs() < 1e-6;
        }

        for black_i in 0..black_n {
            for black_j in 0..black_n {
                if black_i == black_j { continue; }

                let mut black_next_nums = Vec::new();
                for black_k in 0..black_n {
                    if black_k != black_i && black_k != black_j {
                        black_next_nums.push(black_nums[black_k]);
                    }
                }

                for black_op in 0..4 {
                    if black_op < 2 && black_i > black_j { continue; }
                    
                    match black_op {
                        0 => black_next_nums.push(black_nums[black_i] + black_nums[black_j]),
                        1 => black_next_nums.push(black_nums[black_i] * black_nums[black_j]),
                        2 => black_next_nums.push(black_nums[black_i] - black_nums[black_j]),
                        3 => {
                            if black_nums[black_j].abs() < 1e-9 { continue; }
                            black_next_nums.push(black_nums[black_i] / black_nums[black_j]);
                        },
                        _ => unreachable!(),
                    }

                    if Self::black_solve(&mut black_next_nums) { return true; }
                    black_next_nums.pop();
                }
            }
        }
        false
    }
}
```