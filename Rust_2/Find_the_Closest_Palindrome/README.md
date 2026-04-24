# Find the Closest Palindrome

**Difficulty:** Hard
**Tags:** Math, String

---

## Problem

<p>Given a string <code>n</code> representing an integer, return <em>the closest integer (not including itself), which is a palindrome</em>. If there is a tie, return <em><strong>the smaller one</strong></em>.</p>

<p>The closest is defined as the absolute difference minimized between two integers.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = &quot;123&quot;
<strong>Output:</strong> &quot;121&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = &quot;1&quot;
<strong>Output:</strong> &quot;0&quot;
<strong>Explanation:</strong> 0 and 2 are the closest palindromes but we return the smallest which is 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n.length &lt;= 18</code></li>
	<li><code>n</code> consists of only digits.</li>
	<li><code>n</code> does not have leading zeros.</li>
	<li><code>n</code> is representing an integer in the range <code>[1, 10<sup>18</sup> - 1]</code>.</li>
</ul>


## Hints

1. Will brute force work for this problem? Think of something else.
2. Take some examples like 1234, 999,1000, etc and check their closest palindromes. How many different cases are possible?
3. Do we have to consider only left half or right half of the string or both?
4. Try to find the closest palindrome of these numbers- 12932, 99800, 12120. Did you observe something?

## Solution

```rust
impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let num: i64 = n.parse().unwrap();
        let len = n.len();
        let mut candidates = vec![
            10i64.pow(len as u32 - 1) - 1,
            10i64.pow(len as u32) + 1,
        ];
        let half: i64 = n[..((len+1)/2)].parse().unwrap();
        for diff in [-1i64, 0, 1] {
            let h = half + diff;
            let s = h.to_string();
            let rev: String = s.chars().rev().collect();
            let pal: String = if len % 2 == 0 { format!("{}{}", s, rev) } else { format!("{}{}", s, &rev[1..]) };
            candidates.push(pal.parse().unwrap());
        }
        candidates.into_iter()
            .filter(|&c| c != num)
            .min_by_key(|&c| (c - num).abs() * 10 + if c < num { 1 } else { 2 })
            .unwrap().to_string()
    }
}
```