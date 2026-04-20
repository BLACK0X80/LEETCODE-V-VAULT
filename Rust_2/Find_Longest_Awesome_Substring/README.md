# Find Longest Awesome Substring

**Difficulty:** Hard
**Tags:** Hash Table, String, Bit Manipulation

---

## Problem

<p>You are given a string <code>s</code>. An <strong>awesome</strong> substring is a non-empty substring of <code>s</code> such that we can make any number of swaps in order to make it a palindrome.</p>

<p>Return <em>the length of the maximum length <strong>awesome substring</strong> of</em> <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;3242415&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> &quot;24241&quot; is the longest awesome substring, we can form the palindrome &quot;24142&quot; with some swaps.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;12345678&quot;
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;213123&quot;
<strong>Output:</strong> 6
<strong>Explanation:</strong> &quot;213123&quot; is the longest awesome substring, we can form the palindrome &quot;231132&quot; with some swaps.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists only of digits.</li>
</ul>


## Hints

1. Given the character counts, under what conditions can a palindrome be formed ?
2. From left to right, use bitwise xor-operation to compute for any prefix the number of times modulo 2 of each digit.  (mask ^= (1<<(s[i]-'0')).
3. Expected complexity is O(n*A) where A is the alphabet (10).

## Solution

```rust
impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut first_seen = [-2; 1024];
        first_seen[0] = -1;
        let mut mask = 0;
        let mut max_len = 0;
        for (i, c) in s.chars().enumerate() {
            mask ^= 1 << (c as u8 - b'0');
            if first_seen[mask] != -2 {
                max_len = max_len.max(i as i32 - first_seen[mask]);
            } else {
                first_seen[mask] = i as i32;
            }
            for j in 0..10 {
                let target = mask ^ (1 << j);
                if first_seen[target] != -2 {
                    max_len = max_len.max(i as i32 - first_seen[target]);
                }
            }
        }
        max_len
    }
}
```