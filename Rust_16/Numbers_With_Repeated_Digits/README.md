# Numbers With Repeated Digits

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>Given an integer <code>n</code>, return <em>the number of positive integers in the range </em><code>[1, n]</code><em> that have <strong>at least one</strong> repeated digit</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 20
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only positive number (&lt;= 20) with at least 1 repeated digit is 11.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 100
<strong>Output:</strong> 10
<strong>Explanation:</strong> The positive numbers (&lt;= 100) with atleast 1 repeated digit are 11, 22, 33, 44, 55, 66, 77, 88, 99, and 100.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 1000
<strong>Output:</strong> 262
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. How many numbers with no duplicate digits?  How many numbers with K digits and no duplicates?
2. How many numbers with same length as N?  How many numbers with same prefix as N?

## Solution

```rust
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        n - count_no_dup(n)
    }
}

fn count_no_dup(n: i32) -> i32 {
    let digits: Vec<i32> = n.to_string().bytes().map(|b| (b - b'0') as i32).collect();
    let len = digits.len() as i32;
    let mut res = 0;

    
    for l in 1..len {
        res += 9 * (0..l-1).fold(1i32, |acc, i| acc * (9 - i));
    }

    
    let mut used = 0u32;
    for (i, &d) in digits.iter().enumerate() {
        let i = i as i32;
        let lo = if i == 0 { 1 } else { 0 };
        for x in lo..d {
            if used & (1 << x) == 0 {
                let remaining = len - i - 1;
                let available = 10 - i - 1;
                let ways = (0..remaining).fold(1i32, |acc, j| acc * (available - j));
                res += ways;
            }
        }
        if used & (1 << d) != 0 { break; }
        used |= 1 << d;
        if i == len - 1 { res += 1; }
    }

    res
}
```