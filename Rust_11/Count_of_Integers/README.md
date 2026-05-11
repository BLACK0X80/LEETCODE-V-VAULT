# Count of Integers

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming

---

## Problem

<p>You are given two numeric strings <code>num1</code> and <code>num2</code> and two integers <code>max_sum</code> and <code>min_sum</code>. We denote an integer <code>x</code> to be <em>good</em> if:</p>

<ul>
	<li><code>num1 &lt;= x &lt;= num2</code></li>
	<li><code>min_sum &lt;= digit_sum(x) &lt;= max_sum</code>.</li>
</ul>

<p>Return <em>the number of good integers</em>. Since the answer may be large, return it modulo <code>10<sup>9</sup> + 7</code>.</p>

<p>Note that <code>digit_sum(x)</code> denotes the sum of the digits of <code>x</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num1 = &quot;1&quot;, num2 = &quot;12&quot;, <code>min_sum</code> = 1, max_sum = 8
<strong>Output:</strong> 11
<strong>Explanation:</strong> There are 11 integers whose sum of digits lies between 1 and 8 are 1,2,3,4,5,6,7,8,10,11, and 12. Thus, we return 11.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num1 = &quot;1&quot;, num2 = &quot;5&quot;, <code>min_sum</code> = 1, max_sum = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong> The 5 integers whose sum of digits lies between 1 and 5 are 1,2,3,4, and 5. Thus, we return 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num1 &lt;= num2 &lt;= 10<sup>22</sup></code></li>
	<li><code>1 &lt;= min_sum &lt;= max_sum &lt;= 400</code></li>
</ul>


## Hints

1. Let f(n, l, r) denotes the number of integers from 1 to n with the sum of digits between l and r.
2. The answer is f(num2, min_sum, max_sum) - f(num1-1, min_sum, max_sum).
3. You can calculate f(n, l, r) using digit dp.

## Solution

```rust
impl Solution {
    pub fn count(black_num1: String, black_num2: String, black_min_sum: i32, black_max_sum: i32) -> i32 {
        let mut black_s1 = black_num1.into_bytes();
        let black_s2 = black_num2.into_bytes();
        while black_s1.len() < black_s2.len() { black_s1.insert(0, b'0'); }
        
        let black_n = black_s2.len();
        let mut black_memo = vec![vec![vec![vec![-1; 401]; 2]; 2]; black_n];

        fn black_solve(black_idx: usize, black_t1: bool, black_t2: bool, black_sum: i32, black_s1: &[u8], black_s2: &[u8], black_min_s: i32, black_max_s: i32, black_memo: &mut Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
            if black_sum > black_max_s { return 0; }
            if black_idx == black_s2.len() { return if black_sum >= black_min_s { 1 } else { 0 }; }
            
            if black_memo[black_idx][black_t1 as usize][black_t2 as usize][black_sum as usize] != -1 {
                return black_memo[black_idx][black_t1 as usize][black_t2 as usize][black_sum as usize];
            }

            let black_low = if black_t1 { (black_s1[black_idx] - b'0') as i32 } else { 0 };
            let black_high = if black_t2 { (black_s2[black_idx] - b'0') as i32 } else { 9 };
            let mut black_ans = 0;

            for black_d in black_low..=black_high {
                black_ans = (black_ans + black_solve(
                    black_idx + 1,
                    black_t1 && (black_d == black_low),
                    black_t2 && (black_d == black_high),
                    black_sum + black_d,
                    black_s1, black_s2, black_min_s, black_max_s, black_memo
                )) % 1_000_000_007;
            }

            black_memo[black_idx][black_t1 as usize][black_t2 as usize][black_sum as usize] = black_ans;
            black_ans
        }

        black_solve(0, true, true, 0, &black_s1, &black_s2, black_min_sum, black_max_sum, &mut black_memo)
    }
}
```