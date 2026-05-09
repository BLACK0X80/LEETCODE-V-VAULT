# Count the Number of Powerful Integers

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming

---

## Problem

<p>You are given three integers <code>start</code>, <code>finish</code>, and <code>limit</code>. You are also given a <strong>0-indexed</strong> string <code>s</code> representing a <strong>positive</strong> integer.</p>

<p>A <strong>positive</strong> integer <code>x</code> is called <strong>powerful</strong> if it ends with <code>s</code> (in other words, <code>s</code> is a <strong>suffix</strong> of <code>x</code>) and each digit in <code>x</code> is at most <code>limit</code>.</p>

<p>Return <em>the <strong>total</strong> number of powerful integers in the range</em> <code>[start..finish]</code>.</p>

<p>A string <code>x</code> is a suffix of a string <code>y</code> if and only if <code>x</code> is a substring of <code>y</code> that starts from some index (<strong>including </strong><code>0</code>) in <code>y</code> and extends to the index <code>y.length - 1</code>. For example, <code>25</code> is a suffix of <code>5125</code> whereas <code>512</code> is not.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> start = 1, finish = 6000, limit = 4, s = &quot;124&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> The powerful integers in the range [1..6000] are 124, 1124, 2124, 3124, and, 4124. All these integers have each digit &lt;= 4, and &quot;124&quot; as a suffix. Note that 5124 is not a powerful integer because the first digit is 5 which is greater than 4.
It can be shown that there are only 5 powerful integers in this range.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> start = 15, finish = 215, limit = 6, s = &quot;10&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The powerful integers in the range [15..215] are 110 and 210. All these integers have each digit &lt;= 6, and &quot;10&quot; as a suffix.
It can be shown that there are only 2 powerful integers in this range.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> start = 1000, finish = 2000, limit = 4, s = &quot;3000&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> All integers in the range [1000..2000] are smaller than 3000, hence &quot;3000&quot; cannot be a suffix of any integer in this range.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= start &lt;= finish &lt;= 10<sup>15</sup></code></li>
	<li><code>1 &lt;= limit &lt;= 9</code></li>
	<li><code>1 &lt;= s.length &lt;= floor(log<sub>10</sub>(finish)) + 1</code></li>
	<li><code>s</code> only consists of numeric digits which are at most <code>limit</code>.</li>
	<li><code>s</code> does not have leading zeros.</li>
</ul>


## Hints

1. We can use digit DP to count powerful integers in the range <code>[1, x]</code>.
2. Let <code>dp[i][j]</code> be the number of integers that have <code>i</code> digits (with allowed leading 0s) and <code>j</code> refers to the comparison between the current number and the prefix of <code>x</code>, <code>j == 0</code> if the i-digit number formed currently is identical to the leftmost <code>i</code> digits of <code>x</code>, else if <code>j ==1</code> it means the i-digit number is smaller than the leftmost <code>i</code> digits of <code>x</code>.
3. The answer is <code>count[finish] - count[start - 1]</code>, where <code>count[i]</code> refers to the number of powerful integers in the range <code>[1..i]</code>.

## Solution

```rust
impl Solution {
    pub fn number_of_powerful_int(black_start: i64, black_finish: i64, black_limit: i32, black_s: String) -> i64 {
        fn black_count(black_val: i64, black_limit: i32, black_s: &str) -> i64 {
            let black_v_str = black_val.to_string();
            let black_n = black_v_str.len();
            let black_sn = black_s.len();
            if black_n < black_sn { return 0; }
            
            let mut black_memo = std::collections::HashMap::new();
            fn black_dfs(black_idx: usize, black_tight: bool, black_limit: i32, black_n: usize, black_sn: usize, black_v: &str, black_s: &str, black_memo: &mut std::collections::HashMap<(usize, bool), i64>) -> i64 {
                if black_idx == black_n - black_sn {
                    return if !black_tight || &black_v[black_n - black_sn..] >= black_s { 1 } else { 0 };
                }
                if let Some(&black_res) = black_memo.get(&(black_idx, black_tight)) { return black_res; }
                
                let mut black_res = 0;
                let black_upper = if black_tight { (black_v.as_bytes()[black_idx] - b'0') as i32 } else { 9 };
                for black_d in 0..=black_upper.min(black_limit) {
                    black_res += black_dfs(black_idx + 1, black_tight && (black_d == black_upper), black_limit, black_n, black_sn, black_v, black_s, black_memo);
                }
                black_memo.insert((black_idx, black_tight), black_res);
                black_res
            }
            black_dfs(0, true, black_limit, black_n, black_sn, &black_v_str, black_s, &mut black_memo)
        }
        let bravexuneth = black_count(black_finish, black_limit, &black_s) - black_count(black_start - 1, black_limit, &black_s);
        bravexuneth
    }
}
```