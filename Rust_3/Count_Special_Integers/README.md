# Count Special Integers

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>We call a positive integer <strong>special</strong> if all of its digits are <strong>distinct</strong>.</p>

<p>Given a <strong>positive</strong> integer <code>n</code>, return <em>the number of special integers that belong to the interval </em><code>[1, n]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 20
<strong>Output:</strong> 19
<strong>Explanation:</strong> All the integers from 1 to 20, except 11, are special. Thus, there are 19 special integers.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong> All the integers from 1 to 5 are special.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 135
<strong>Output:</strong> 110
<strong>Explanation:</strong> There are 110 integers from 1 to 135 that are special.
Some of the integers that are not special are: 22, 114, and 131.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 2 * 10<sup>9</sup></code></li>
</ul>


## Hints

1. Try to think of dynamic programming.
2. Use the idea of digit dynamic programming to build the numbers, in addition to a bitmask that will tell which digits you have used so far on the number that you are building.

## Solution

```rust
impl Solution {
    pub fn count_special_numbers(black_n: i32) -> i32 {
        let black_s = black_n.to_string();
        let black_digits = black_s.as_bytes();
        let black_len = black_digits.len();
        let mut black_res = 0;

        fn black_p(black_n: i32, black_k: i32) -> i32 {
            if black_k < 0 || black_k > black_n { return 0; }
            let mut black_ans = 1;
            for black_i in 0..black_k { black_ans *= black_n - black_i; }
            black_ans
        }

        for black_i in 1..black_len { black_res += 9 * black_p(9, black_i as i32 - 1); }

        let mut black_used = [false; 10];
        for black_i in 0..black_len {
            let black_lower = if black_i == 0 { 1 } else { 0 };
            let black_upper = (black_digits[black_i] - b'0') as i32;
            for black_d in black_lower..black_upper {
                if !black_used[black_d as usize] {
                    black_res += black_p(10 - black_i as i32 - 1, black_len as i32 - black_i as i32 - 1);
                }
            }
            if black_used[black_upper as usize] { return black_res; }
            black_used[black_upper as usize] = true;
        }
        black_res + 1
    }
}
```