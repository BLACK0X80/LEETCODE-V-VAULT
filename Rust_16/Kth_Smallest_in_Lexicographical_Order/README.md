# K-th Smallest in Lexicographical Order

**Difficulty:** Hard
**Tags:** Trie

---

## Problem

<p>Given two integers <code>n</code> and <code>k</code>, return <em>the</em> <code>k<sup>th</sup></code> <em>lexicographically smallest integer in the range</em> <code>[1, n]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 13, k = 2
<strong>Output:</strong> 10
<strong>Explanation:</strong> The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1, k = 1
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let count = |mut cur: i64, n: i64| -> i64 {
            let mut next = cur + 1;
            let mut cnt = 0i64;
            while cur <= n {
                cnt += (n + 1).min(next) - cur;
                cur *= 10; next *= 10;
            }
            cnt
        };
        let (n, mut k) = (n as i64, k as i64);
        let mut cur = 1i64;
        k -= 1;
        while k > 0 {
            let cnt = count(cur, n);
            if k >= cnt { k -= cnt; cur += 1; }
            else { k -= 1; cur *= 10; }
        }
        cur as i32
    }
}
```