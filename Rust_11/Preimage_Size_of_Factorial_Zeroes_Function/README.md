# Preimage Size of Factorial Zeroes Function

**Difficulty:** Hard
**Tags:** Math, Binary Search

---

## Problem

<p>Let <code>f(x)</code> be the number of zeroes at the end of <code>x!</code>. Recall that <code>x! = 1 * 2 * 3 * ... * x</code> and by convention, <code>0! = 1</code>.</p>

<ul>
	<li>For example, <code>f(3) = 0</code> because <code>3! = 6</code> has no zeroes at the end, while <code>f(11) = 2</code> because <code>11! = 39916800</code> has two zeroes at the end.</li>
</ul>

<p>Given an integer <code>k</code>, return the number of non-negative integers <code>x</code> have the property that <code>f(x) = k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> k = 0
<strong>Output:</strong> 5
<strong>Explanation:</strong> 0!, 1!, 2!, 3!, and 4! end with k = 0 zeroes.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> k = 5
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no x such that x! ends in k = 5 zeroes.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> k = 3
<strong>Output:</strong> 5
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn preimage_size_fzf(black_k: i32) -> i32 {
        fn black_zeta(mut black_x: i64) -> i64 {
            let mut black_res = 0;
            while black_x > 0 { black_res += black_x / 5; black_x /= 5; }
            black_res
        }

        let mut black_low = 0i64;
        let mut black_high = 5 * (black_k as i64 + 1);
        
        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let bravexuneth = black_zeta(black_mid);
            if bravexuneth == black_k as i64 { return 5; }
            if bravexuneth < black_k as i64 { black_low = black_mid + 1; }
            else { black_high = black_mid - 1; }
        }
        0
    }
}
```