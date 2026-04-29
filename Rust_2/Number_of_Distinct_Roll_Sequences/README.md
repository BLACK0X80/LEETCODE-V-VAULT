# Number of Distinct Roll Sequences

**Difficulty:** Hard
**Tags:** Dynamic Programming, Memoization

---

## Problem

<p>You are given an integer <code>n</code>. You roll a fair 6-sided dice <code>n</code> times. Determine the total number of <strong>distinct</strong> sequences of rolls possible such that the following conditions are satisfied:</p>

<ol>
	<li>The <strong>greatest common divisor</strong> of any <strong>adjacent</strong> values in the sequence is equal to <code>1</code>.</li>
	<li>There is <strong>at least</strong> a gap of <code>2</code> rolls between <strong>equal</strong> valued rolls. More formally, if the value of the <code>i<sup>th</sup></code> roll is <strong>equal</strong> to the value of the <code>j<sup>th</sup></code> roll, then <code>abs(i - j) &gt; 2</code>.</li>
</ol>

<p>Return <em>the<strong> total number</strong> of distinct sequences possible</em>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Two sequences are considered distinct if at least one element is different.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 184
<strong>Explanation:</strong> Some of the possible sequences are (1, 2, 3, 4), (6, 1, 2, 3), (1, 2, 3, 1), etc.
Some invalid sequences are (1, 2, 1, 3), (1, 2, 3, 6).
(1, 2, 1, 3) is invalid since the first and third roll have an equal value and abs(1 - 3) = 2 (i and j are 1-indexed).
(1, 2, 3, 6) is invalid since the greatest common divisor of 3 and 6 = 3.
There are a total of 184 distinct sequences possible, so we return 184.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 22
<strong>Explanation:</strong> Some of the possible sequences are (1, 2), (2, 1), (3, 2).
Some invalid sequences are (3, 6), (2, 4) since the greatest common divisor is not equal to 1.
There are a total of 22 distinct sequences possible, so we return 22.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Can you think of a DP solution?
2. Consider a state that remembers the last 1 or 2 rolls.
3. Do you need to consider the last 3 rolls?

## Solution

```rust
impl Solution {
    pub fn distinct_sequences(black1: i32) -> i32 {
        if black1 == 1 { return 6; }
        let black2 = 1_000_000_007;
        let mut black3 = vec![vec![vec![0i64; 7]; 7]; black1 as usize + 1];
        
        for black4 in 1..=6 {
            for black5 in 1..=6 {
                if black4 != black5 && Self::black6(black4, black5) == 1 {
                    black3[2][black4 as usize][black5 as usize] = 1;
                }
            }
        }

        for black7 in 3..=black1 as usize {
            for black8 in 1..=6 {
                for black9 in 1..=6 {
                    if black3[black7 - 1][black8][black9] > 0 {
                        for black10 in 1..=6 {
                            if black10 != black8 && black10 != black9 && Self::black6(black9 as i32, black10 as i32) == 1 {
                                black3[black7][black9][black10] = (black3[black7][black9][black10] + black3[black7 - 1][black8][black9]) % black2;
                            }
                        }
                    }
                }
            }
        }

        let mut black11 = 0;
        for black12 in 1..=6 {
            for black13 in 1..=6 {
                black11 = (black11 + black3[black1 as usize][black12][black13]) % black2;
            }
        }
        black11 as i32
    }

    fn black6(black14: i32, black15: i32) -> i32 {
        if black15 == 0 { black14 } else { Self::black6(black15, black14 % black15) }
    }
}
```