# Substring With Largest Variance

**Difficulty:** Hard
**Tags:** Hash Table, String, Dynamic Programming, Enumeration

---

## Problem

<p>The <strong>variance</strong> of a string is defined as the largest difference between the number of occurrences of <strong>any</strong> <code>2</code> characters present in the string. Note the two characters may or may not be the same.</p>

<p>Given a string <code>s</code> consisting of lowercase English letters only, return <em>the <strong>largest variance</strong> possible among all <strong>substrings</strong> of</em> <code>s</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters within a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aababbb&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong>
All possible variances along with their respective substrings are listed below:
- Variance 0 for substrings &quot;a&quot;, &quot;aa&quot;, &quot;ab&quot;, &quot;abab&quot;, &quot;aababb&quot;, &quot;ba&quot;, &quot;b&quot;, &quot;bb&quot;, and &quot;bbb&quot;.
- Variance 1 for substrings &quot;aab&quot;, &quot;aba&quot;, &quot;abb&quot;, &quot;aabab&quot;, &quot;ababb&quot;, &quot;aababbb&quot;, and &quot;bab&quot;.
- Variance 2 for substrings &quot;aaba&quot;, &quot;ababbb&quot;, &quot;abbb&quot;, and &quot;babb&quot;.
- Variance 3 for substring &quot;babbb&quot;.
Since the largest possible variance is 3, we return it.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcde&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong>
No letter occurs more than once in s, so the variance of every substring is 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Think about how to solve the problem if the string had only two distinct characters.
2. If we replace all occurrences of the first character by +1 and those of the second character by -1, can we efficiently calculate the largest possible variance of a string with only two distinct characters?
3. Now, try finding the optimal answer by taking all possible pairs of characters into consideration.

## Solution

```rust
impl Solution {
    pub fn largest_variance(black1: String) -> i32 {
        let black2 = black1.as_bytes();
        let mut black3 = 0;
        let mut black4 = vec![0; 26];
        for &black5 in black2 { black4[(black5 - b'a') as usize] += 1; }

        for black6 in 0..26 {
            for black7 in 0..26 {
                if black6 == black7 || black4[black6] == 0 || black4[black7] == 0 { continue; }
                let (mut black8, mut black9) = (0, 0);
                let mut black10 = false;
                for &black11 in black2 {
                    let black12 = (black11 - b'a') as usize;
                    if black12 == black6 { black8 += 1; }
                    else if black12 == black7 {
                        black9 += 1;
                        black10 = true;
                    }
                    if black9 > 0 { black3 = black3.max(black8 - black9); }
                    else if black10 { black3 = black3.max(black8 - 1); }
                    
                    if black8 < black9 {
                        black8 = 0;
                        black9 = 0;
                    }
                }
            }
        }
        black3
    }
}
```