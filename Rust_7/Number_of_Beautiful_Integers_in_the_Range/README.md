# Number of Beautiful Integers in the Range

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>You are given positive integers <code>low</code>, <code>high</code>, and <code>k</code>.</p>

<p>A number is <strong>beautiful</strong> if it meets both of the following conditions:</p>

<ul>
	<li>The count of even digits in the number is equal to the count of odd digits.</li>
	<li>The number is divisible by <code>k</code>.</li>
</ul>

<p>Return <em>the number of beautiful integers in the range</em> <code>[low, high]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> low = 10, high = 20, k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 beautiful integers in the given range: [12,18]. 
- 12 is beautiful because it contains 1 odd digit and 1 even digit, and is divisible by k = 3.
- 18 is beautiful because it contains 1 odd digit and 1 even digit, and is divisible by k = 3.
Additionally we can see that:
- 16 is not beautiful because it is not divisible by k = 3.
- 15 is not beautiful because it does not contain equal counts even and odd digits.
It can be shown that there are only 2 beautiful integers in the given range.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> low = 1, high = 10, k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is 1 beautiful integer in the given range: [10].
- 10 is beautiful because it contains 1 odd digit and 1 even digit, and is divisible by k = 1.
It can be shown that there is only 1 beautiful integer in the given range.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> low = 5, high = 5, k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are 0 beautiful integers in the given range.
- 5 is not beautiful because it is not divisible by k = 2 and it does not contain equal even and odd digits.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt; low &lt;= high &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt; k &lt;= 20</code></li>
</ul>


## Hints

1. <div class="_1l1MA">The intended solution uses Dynamic Programming.</div>
2. <div class="_1l1MA">Let <code> f(n) </code> denote number of beautiful integers in the range <code> [1…n] </code>, then the answer is <code> f(r) - f(l-1) </code>.</div>

## Solution

```rust
impl Solution {
    pub fn number_of_beautiful_integers(black_low: i32, black_high: i32, black_k: i32) -> i32 {
        fn black_count(black_val: i32, black_k: i32) -> i32 {
            if black_val <= 0 { return 0; }
            let black_s = black_val.to_string();
            let black_n = black_s.len();
            let mut black_memo = std::collections::HashMap::new();
            
            fn black_dfs(black_idx: usize, black_tight: bool, black_lead: bool, black_diff: i32, black_rem: i32, black_k: i32, black_n: usize, black_s: &str, black_memo: &mut std::collections::HashMap<(usize, bool, bool, i32, i32), i32>) -> i32 {
                if black_idx == black_n { return if !black_lead && black_diff == 10 && black_rem == 0 { 1 } else { 0 }; }
                let black_state = (black_idx, black_tight, black_lead, black_diff, black_rem);
                if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }
                
                let mut black_res = 0;
                let black_upper = if black_tight { (black_s.as_bytes()[black_idx] - b'0') as i32 } else { 9 };
                for black_d in 0..=black_upper {
                    let black_n_lead = black_lead && black_d == 0;
                    let black_n_diff = if black_n_lead { 10 } else if black_d % 2 == 0 { black_diff + 1 } else { black_diff - 1 };
                    black_res += black_dfs(black_idx + 1, black_tight && (black_d == black_upper), black_n_lead, black_n_diff, (black_rem * 10 + black_d) % black_k, black_k, black_n, black_s, black_memo);
                }
                black_memo.insert(black_state, black_res);
                black_res
            }
            black_dfs(0, true, true, 10, 0, black_k, black_n, &black_s, &mut black_memo)
        }
        let bravexuneth = black_count(black_high, black_k) - black_count(black_low - 1, black_k);
        bravexuneth
    }
}
```