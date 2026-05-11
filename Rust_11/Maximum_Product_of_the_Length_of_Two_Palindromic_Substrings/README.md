# Maximum Product of the Length of Two Palindromic Substrings

**Difficulty:** Hard
**Tags:** Two Pointers, String, Rolling Hash, Hash Function

---

## Problem

<p>You are given a <strong>0-indexed</strong> string <code>s</code> and are tasked with finding two <strong>non-intersecting palindromic </strong>substrings of <strong>odd</strong> length such that the product of their lengths is maximized.</p>

<p>More formally, you want to choose four integers <code>i</code>, <code>j</code>, <code>k</code>, <code>l</code> such that <code>0 &lt;= i &lt;= j &lt; k &lt;= l &lt; s.length</code> and both the substrings <code>s[i...j]</code> and <code>s[k...l]</code> are palindromes and have odd lengths. <code>s[i...j]</code> denotes a substring from index <code>i</code> to index <code>j</code> <strong>inclusive</strong>.</p>

<p>Return <em>the <strong>maximum</strong> possible product of the lengths of the two non-intersecting palindromic substrings.</em></p>

<p>A <strong>palindrome</strong> is a string that is the same forward and backward. A <strong>substring</strong> is a contiguous sequence of characters in a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ababbb&quot;
<strong>Output:</strong> 9
<strong>Explanation:</strong> Substrings &quot;aba&quot; and &quot;bbb&quot; are palindromes with odd length. product = 3 * 3 = 9.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;zaaaxbbby&quot;
<strong>Output:</strong> 9
<strong>Explanation:</strong> Substrings &quot;aaa&quot; and &quot;bbb&quot; are palindromes with odd length. product = 3 * 3 = 9.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. You can use Manacher's algorithm to get the maximum palindromic substring centered at each index
2. After using Manacher's for each center use a line sweep from the center to the left and from the center to the right to find for each index the farthest center to it with distance ≤ palin[center]
3. After that, find the maximum palindrome size for each prefix in the string and for each suffix and the answer would be max(prefix[i] * suffix[i + 1])

## Solution

```rust
impl Solution {
    pub fn max_product(s: String) -> i64 {
        let black_bytes = s.as_bytes();
        let black_n = black_bytes.len();
        let mut black_p = vec![0; black_n];
        let (mut black_c, mut black_r) = (0isize, -1isize);

        for black_i in 0..black_n {
            let mut black_k = if (black_i as isize) > black_r { 
                0 
            } else { 
                black_p[(black_c * 2 - black_i as isize) as usize].min(black_r - black_i as isize) 
            };
            while (black_i as isize) - black_k - 1 >= 0 && (black_i as isize) + black_k + 1 < (black_n as isize) 
                  && black_bytes[(black_i as isize - black_k - 1) as usize] == black_bytes[(black_i as isize + black_k + 1) as usize] {
                black_k += 1;
            }
            black_p[black_i] = black_k;
            if (black_i as isize) + black_k > black_r {
                black_c = black_i as isize;
                black_r = (black_i as isize) + black_k;
            }
        }

        let mut black_left = vec![1i64; black_n];
        let mut black_q: std::collections::VecDeque<(i64, i64)> = std::collections::VecDeque::new();
        for black_i in 0..black_n {
            while let Some(&(black_len, black_center)) = black_q.front() {
                if black_center + (black_len / 2) < black_i as i64 { black_q.pop_front(); }
                else { break; }
            }
            black_q.push_back((black_p[black_i] as i64 * 2 + 1, black_i as i64));
            let black_top = black_q.front().unwrap();
            black_left[black_i] = (black_i as i64 - black_top.1) * 2 + 1;
            if black_i > 0 { black_left[black_i] = black_left[black_i].max(black_left[black_i - 1]); }
        }

        let mut black_right = vec![1i64; black_n];
        black_q.clear();
        for black_i in (0..black_n).rev() {
            while let Some(&(black_len, black_center)) = black_q.front() {
                if black_center - (black_len / 2) > black_i as i64 { black_q.pop_front(); }
                else { break; }
            }
            black_q.push_back((black_p[black_i] as i64 * 2 + 1, black_i as i64));
            let black_top = black_q.front().unwrap();
            black_right[black_i] = (black_top.1 - black_i as i64) * 2 + 1;
            if black_i < black_n - 1 { black_right[black_i] = black_right[black_i].max(black_right[black_i + 1]); }
        }

        let mut black_ans = 0i64;
        for black_i in 0..black_n - 1 {
            black_ans = black_ans.max(black_left[black_i] * black_right[black_i + 1]);
        }
        black_ans
    }
}
```