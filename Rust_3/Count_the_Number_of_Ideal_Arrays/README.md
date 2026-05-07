# Count the Number of Ideal Arrays

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Combinatorics, Number Theory

---

## Problem

<p>You are given two integers <code>n</code> and <code>maxValue</code>, which are used to describe an <strong>ideal</strong> array.</p>

<p>A <strong>0-indexed</strong> integer array <code>arr</code> of length <code>n</code> is considered <strong>ideal</strong> if the following conditions hold:</p>

<ul>
	<li>Every <code>arr[i]</code> is a value from <code>1</code> to <code>maxValue</code>, for <code>0 &lt;= i &lt; n</code>.</li>
	<li>Every <code>arr[i]</code> is divisible by <code>arr[i - 1]</code>, for <code>0 &lt; i &lt; n</code>.</li>
</ul>

<p>Return <em>the number of <strong>distinct</strong> ideal arrays of length </em><code>n</code>. Since the answer may be very large, return it modulo <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2, maxValue = 5
<strong>Output:</strong> 10
<strong>Explanation:</strong> The following are the possible ideal arrays:
- Arrays starting with the value 1 (5 arrays): [1,1], [1,2], [1,3], [1,4], [1,5]
- Arrays starting with the value 2 (2 arrays): [2,2], [2,4]
- Arrays starting with the value 3 (1 array): [3,3]
- Arrays starting with the value 4 (1 array): [4,4]
- Arrays starting with the value 5 (1 array): [5,5]
There are a total of 5 + 2 + 1 + 1 + 1 = 10 distinct ideal arrays.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 5, maxValue = 3
<strong>Output:</strong> 11
<strong>Explanation:</strong> The following are the possible ideal arrays:
- Arrays starting with the value 1 (9 arrays): 
   - With no other distinct values (1 array): [1,1,1,1,1] 
   - With 2<sup>nd</sup> distinct value 2 (4 arrays): [1,1,1,1,2], [1,1,1,2,2], [1,1,2,2,2], [1,2,2,2,2]
   - With 2<sup>nd</sup> distinct value 3 (4 arrays): [1,1,1,1,3], [1,1,1,3,3], [1,1,3,3,3], [1,3,3,3,3]
- Arrays starting with the value 2 (1 array): [2,2,2,2,2]
- Arrays starting with the value 3 (1 array): [3,3,3,3,3]
There are a total of 9 + 1 + 1 = 11 distinct ideal arrays.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= maxValue &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Notice that an ideal array is non-decreasing.
2. Consider an alternative problem: where an ideal array must also be strictly increasing. Can you use DP to solve it?
3. Will combinatorics help to get an answer from the alternative problem to the actual problem?

## Solution

```rust
impl Solution {
    pub fn ideal_arrays(black1: i32, black2: i32) -> i32 {
        let black3 = 1_000_000_007i64;
        let mut black4 = vec![vec![0i64; 15]; black2 as usize + 1];
        for i in 1..=black2 as usize { black4[i][1] = 1; }

        for j in 1..14 {
            for i in 1..=black2 as usize {
                if black4[i][j] == 0 { continue; }
                for k in (i * 2..=black2 as usize).step_by(i) {
                    black4[k][j + 1] = (black4[k][j + 1] + black4[i][j]) % black3;
                }
            }
        }

        let mut black5 = vec![vec![0i64; 15]; black1 as usize + 1];
        for i in 0..=black1 as usize {
            black5[i][0] = 1;
            for j in 1..=i.min(14) {
                black5[i][j] = (black5[i - 1][j - 1] + black5[i - 1][j]) % black3;
            }
        }

        let mut black6 = 0i64;
        for i in 1..=black2 as usize {
            for j in 1..=14.min(black1 as usize) {
                let black7 = (black4[i][j] * black5[black1 as usize - 1][j - 1]) % black3;
                black6 = (black6 + black7) % black3;
            }
        }
        black6 as i32
    }
}
```